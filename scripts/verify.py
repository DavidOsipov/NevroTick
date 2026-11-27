#!/usr/bin/env python3
"""Run the admitted fixture, engine and platform owners in an isolated sandbox.

Acquisition is separate. Missing tools, vendor bytes or product tests fail closed.
Windows cross-linking supplies no native UI, service or release qualification.
"""
from __future__ import annotations

import argparse
from collections.abc import Set
from contextlib import ExitStack
import ctypes
import errno
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import socket
import stat
import sys
import tempfile
import time
import tomllib
from types import ModuleType
from typing import TYPE_CHECKING, Any, BinaryIO
import xml.etree.ElementTree as ET

if TYPE_CHECKING:
    from windows_versioninfo import UiExecutionManifest, VersionInfo

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("product_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
STRICT_SPEC = importlib.util.spec_from_file_location("product_strict", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(STRICT_SPEC)
STRICT_SPEC.loader.exec_module(STRICT)
SLINT_SPEC = importlib.util.spec_from_file_location("slint_dependency_profile", ROOT / "scripts/slint_dependency_profile.py")
SLINT = importlib.util.module_from_spec(SLINT_SPEC)
SLINT_SPEC.loader.exec_module(SLINT)
SDK_SPEC = importlib.util.spec_from_file_location("slint_linux_sdk", ROOT / "scripts/slint_linux_sdk.py")
SDK = importlib.util.module_from_spec(SDK_SPEC)
SDK_SPEC.loader.exec_module(SDK)
ADVISORY_SPEC = importlib.util.spec_from_file_location("advisory_snapshot", ROOT / "scripts/advisory_snapshot.py")
ADVISORY = importlib.util.module_from_spec(ADVISORY_SPEC)
ADVISORY_SPEC.loader.exec_module(ADVISORY)
PACKAGE = "nevrotick-ui"
MEMBER = "crates/nevrotick-ui"
LEGACY_PACKAGES = {PACKAGE: MEMBER, "ntp-wire": "crates/ntp-wire", "time-domain": "crates/time-domain",
            "ntp-exchange": "crates/ntp-exchange", "time-core": "crates/time-core",
            "nevrotick-engine": "crates/nevrotick-engine", "windows-platform": "crates/windows-platform"}
PACKAGES = {**LEGACY_PACKAGES, SLINT.NATIVE_PACKAGE: "crates/" + SLINT.NATIVE_PACKAGE}
HOST_TARGET = "x86_64-unknown-linux-gnu"
RGT_ENGINE_VERSIONS: dict[str, str] = {"ed25519-dalek": "3.0.0", "sha2": "0.11.1"}
REGISTRY_VERSIONS = {
    "castaway": "0.2.4", "cfg-if": "1.0.5", "compact_str": "0.9.1", "garde": "0.23.0",
    "garde_derive": "0.23.0", "getrandom": "0.4.3", "itoa": "1.0.18", "libc": "0.2.191",
    "mio": "1.2.4", "pin-project-lite": "0.2.17", "proc-macro2": "1.0.107", "quote": "1.0.47",
    "rustversion": "1.0.23", "ryu": "1.0.23", "smallvec": "1.16.4", "socket2": "0.6.5",
    "static_assertions": "1.1.0", "syn": "2.0.119", "tokio": "1.54.1", "tokio-macros": "2.7.1",
    "unicode-ident": "1.0.27", "windows-link": "0.2.1", "windows-sys": "0.61.2",
}
METADATA_ONLY_VERSIONS = {"r-efi": "6.0.0", "wasi": "0.11.1+wasi-snapshot-preview1"}
BUILD_SCRIPTS = {"getrandom@0.4.3", "libc@0.2.191", "proc-macro2@1.0.107",
                 "quote@1.0.47", "rustversion@1.0.23"}
# Bound concurrent LLVM test-binary links within the ordinary128-task sandbox.
# Test execution and ten-worker mutation scheduling have separate policies.
TEST_BUILD_JOBS = 6
BOUNDED_TEST_BUILD_PACKAGES = {"nevrotick-ui", "time-core", "ntp-wire", "time-domain"}
# Pin the selected row before replacing its Cargo build with explicit reuse.
ENGINE_EP1_RELEASE_BUILD_ROW = "engine-ep1-release-test-build"
ENGINE_EP1_RELEASE_TEST_ROW = "optimized-tests:nevrotick-engine"
ENGINE_EP1_ARCHIVE_NAME = "gnu-ep1-engine.nextest.tar.zst"
ENGINE_EP1_EXTRACT_NAME = "gnu-ep1-engine-reuse"
ENGINE_EP1_RELEASE_BUILD = (
    "cargo", "--config", 'source.crates-io.replace-with="reviewed"',
    "--config", 'source.reviewed.directory="/vendor"', "nextest", "archive", "--package", "nevrotick-engine",
    "--cargo-profile", "release", "--frozen", "--build-jobs", "6", "--lib", "--tests",
    "--user-config-file", "none", "--features", "nevrotick-engine/ke-association-prototype",
    "--archive-file", f"/work/{ENGINE_EP1_ARCHIVE_NAME}",
)
ENGINE_EP1_RELEASE_TEST = (
    "cargo", "--config", 'source.crates-io.replace-with="reviewed"',
    "--config", 'source.reviewed.directory="/vendor"', "nextest", "run", "--package", "nevrotick-engine",
    "--cargo-profile", "release", "--frozen", "--build-jobs", "6", "--lib", "--tests",
    "--user-config-file", "none", "--no-tests=fail", "--features", "nevrotick-engine/ke-association-prototype",
)
ENGINE_EP1_RELEASE_DISCOVERY = (
    "cargo", "nextest", "list", "--archive-file", f"/work/{ENGINE_EP1_ARCHIVE_NAME}",
    "--extract-to", f"/work/{ENGINE_EP1_EXTRACT_NAME}", "--workspace-remap", "/work/fixture",
    "--user-config-file", "none", "--message-format", "json",
)
ENGINE_EP1_RELEASE_REUSE = (
    "cargo", "nextest", "run",
    "--cargo-metadata", f"/work/{ENGINE_EP1_EXTRACT_NAME}/target/nextest/cargo-metadata.json",
    "--binaries-metadata", f"/work/{ENGINE_EP1_EXTRACT_NAME}/target/nextest/binaries-metadata.json",
    "--target-dir-remap", f"/work/{ENGINE_EP1_EXTRACT_NAME}/target", "--workspace-remap", "/work/fixture",
    "--user-config-file", "none", "--no-tests=fail",
)
CaseIdentity = tuple[str, str]
OPTIMIZED_DISCRIMINATORS = {
    "time-domain": ("time-domain::coordinate", "release_build_preserves_checked_admission"),
    "ntp-exchange": ("ntp-exchange::association", "release_build_preserves_exchange_admission"),
    "time-core": ("time-core::regression", "release_build_preserves_regression_admission"),
    "nevrotick-engine": ("nevrotick-engine::prepare_entered", "prepare_rejects_entered_runtime_before_native_startup"),
}
ENGINE_PROCESSING_GUARD = "processing_guard_tests::processing_uses_fresh_deadline_and_current_control"
ENGINE_RETAINED_REPLY_CASES = {
    "source_policy::tests::managed_series_retains_one_deny_across_final_policy_release",
    "series::tests::managed_serial_reply_preserves_queued_second_datagram",
}
# Nonempty EP1 selection across actual private owners, required in ordinary and
# optimized Engine unit binaries. These are component tests, not native KE proof.
ENGINE_KE_TEST_CASES = {
    "nts_ke_experiment::tests::configuration_admission_and_typed_identity",
    "nts_ke_experiment::tests::strict_deadline_neighbors_and_fixed_expiry",
    "nts_ke_experiment::process::tests::claim_is_permanent_before_and_after_provider_scope",
    "nts_ke_experiment::association::tests::genuine_owner_boundary_moves_cookies_and_consumes_completion_once",
    "nts_ke_experiment::association::tests::foreign_completion_refuses_without_partial_publication",
    "nts_ke_experiment::association::tests::post_ke_cancellation_refuses_before_entropy",
    "nts_ke_experiment::association::tests::association_expiry_after_ke_prevents_preview",
    "nts_ke_experiment::association::tests::published_association_reaches_actual_sealer_and_finishes_once",
    "nts_ke_experiment::association::tests::preview_refusal_does_not_reset_work_credit",
    "nts_ke_experiment::request::tests::complete_request_matches_existing_independent_answer",
    "nts_ke_experiment::request::tests::s2c_changes_do_not_select_the_outgoing_key",
    "nts_ke_experiment::handoff::tests::cancellation_after_sealing_refuses_final_completion",
}
PATCH_TEST_CASES = {
    ("spin_on", "tests::ready"), ("spin_on", "tests::count"),
    ("spin_on::stack_pin", "non_unpin_future_keeps_its_address_and_drops_once_after_pending_polls"),
}
POINTER = Path.home() / ".local/share/nevrotick/ui-m0.json"
SOURCE_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "rustfmt.toml", "clippy.toml",
                "deny.toml", ".cargo/config.toml", ".config/nextest.toml")
CHECKER_FILES = ("scripts/verify.py", "scripts/check-environment.py", "scripts/check-strict.py",
                 "scripts/slint_dependency_profile.py", "scripts/dependency_cooldown.py", "scripts/slint_linux_sdk.py",
                 "scripts/advisory_snapshot.py", "tools/crypto-aes-siv-witness/fuzz/input_custody.py",
                 "scripts/tests/test_verify_vendor_admission.py", "scripts/tests/test_engine_ke_dependency_profile.py")
UI_TEXT_CHECKER_FILES = ("scripts/check-ui-text.py", "scripts/check_ui_text_coverage.py",
                         "scripts/ui_source_io.py",
                         "scripts/slint_text_graph.py", "scripts/ui_text_bindings.py", "scripts/protected_json.py",
                         "scripts/tests/test_ui_text_coverage.py", "scripts/generate-ui-text.py",
                         "scripts/ui_text_codegen.py", "scripts/ui-text-templates/projection-header.rs.in",
                         "scripts/tests/test_ui_text_codegen.py",
                         "scripts/tests/fixtures/ui-text-codegen-contract.json",
                         "scripts/tests/fixtures/ui-text-codegen-output-sha256.json",
                         "scripts/tests/fixtures/ui-text-codegen-contract-v2.json",
                         "scripts/tests/fixtures/ui-text-codegen-output-v2-sha256.json",
                         "scripts/native_text_graph.py", "scripts/tests/test_ui_text_native.py",
                         "scripts/tests/fixtures/ui-text-native-source-v2.json")


UI_NATIVE_INPUT_CHECKER_FILES = ("scripts/native_input_source.py",
                                 "scripts/tests/test_native_input_source.py")

WINDOWS_PE_INPUT_FILES = ("scripts/windows_pe.py", "scripts/tests/test_windows_pe.py")
UI_ENTRY_SOURCE = "crates/nevrotick-ui/startup/cpu_entry.c"
UI_ENTRY_CONTROL_SOURCE = "crates/nevrotick-ui/startup/cpu_entry_control.c"
UI_ENTRY_MAP_LEAF = "nevrotick-ui-entry.map"
UI_ENTRY_MAP = "/work/metadata/" + UI_ENTRY_MAP_LEAF
UI_ENTRY_MAP_MAX_BYTES = 32 * 1024**2
UI_ENTRY_CLANG = Path("/usr/lib/llvm-18/bin/clang")
UI_ENTRY_TOOL_PINS = {
    UI_ENTRY_CLANG: "8ef402d453d1ba4902e4ee0f0f847f6cfa01400c95aa43c24e97818b9c0e3f45",
    UI_ENTRY_CLANG.parent.parent / "lib/libclang-cpp.so.18.1":
        "b9592746f7f8ff1156680fb1fad5faf5486df7a67aca8327cc959463416c6718",
    UI_ENTRY_CLANG.parent.parent / "lib/libLLVM.so.1":
        "5c3a63957e28c15cdf1503892074f032d29b72e36a9824a77dfa87e48a0dafb0",
}
WINDOWS_UI_INPUT_FILES = (
    "metadata/product.toml",
    "metadata/__init__.py",
    "metadata/pipeline/__init__.py",
    "metadata/pipeline/__main__.py",
    "metadata/pipeline/contract.py",
    "metadata/pipeline/delivery.py",
    "metadata/pipeline/fields.py",
    "metadata/pipeline/generate.py",
    "metadata/pipeline/text_policy.py",
    "scripts/windows_versioninfo_expectation.py",
    "scripts/windows_versioninfo.py",
    "scripts/tests/test_windows_versioninfo.py",
)

MSIX_INSPECTION_LANE = "windows-msix-inspect"
MSIX_INSPECTION_MAX_OUTPUT = 16 * 1024
MSIX_AUTHORING_INPUT = "authoring/msix-inputs.json"
MSIX_REQUIRED_AUTHORING = frozenset({MSIX_AUTHORING_INPUT, "authoring/service-inputs.json",
                                     "authoring/event-inputs.json"})
MSIX_ALLOWED_AUTHORING = MSIX_REQUIRED_AUTHORING | {"authoring/service-events.man"}
MSIX_AUTHORING_MAX_BYTES = 1024 * 1024
MSIX_HANDOFF_RECEIPT_MAX_BYTES = 4 * 1024 * 1024
MSIX_PROPERTIES_FRAGMENT = "authoring/msix-properties.xml"
MSIX_FOUNDATION_NAMESPACE = "http://schemas.microsoft.com/appx/manifest/foundation/windows10"
MSIX_UAP10_NAMESPACE = "http://schemas.microsoft.com/appx/manifest/uap/windows10/10"
MSIX_TRUSTED_LAUNCH_NAMESPACE = "http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10"
MSIX_PACKAGE_LOGO = r"Assets\PackageLogo.png"
MSIX_XML_DECLARATION = b'<?xml version="1.0" encoding="utf-8"?>'
MSIX_ARTWORK_FIELDS = {
    "Properties/Logo": ("catalogue.B.6.2.Package/Properties/Logo", "derived"),
    "VisualElements/@Square44x44Logo": (
        "catalogue.B.6.5.uap:VisualElements/@Square44x44Logo", "populated",
    ),
    "VisualElements/@Square150x150Logo": (
        "catalogue.B.6.5.uap:VisualElements/@Square150x150Logo", "populated",
    ),
}
# Base resources independently decoded from the reviewed brand kit. This is a
# source-preparation selection, not an approved production payload inventory.
MSIX_BASE_ARTWORK = {
    "PackageLogo.png": (
        "Properties/Logo", 50, 3873, "2f710c8f4824df9f8986df384a7778071758e6f38f3239788bae2bd0b191da46",
    ),
    "Square44x44Logo.png": (
        "VisualElements/@Square44x44Logo", 44, 4324, "ed6d911e8ba391520bb64ab47d9f6ad026210986f305bb60e6330d2f016c4676",
    ),
    "Square150x150Logo.png": (
        "VisualElements/@Square150x150Logo", 150, 20874, "062c8fed203a15745d9a31668d0e0e092ddba537832e2534e6e9ab5f7bbc7b9e",
    ),
}
# Qualified images use the same three unqualified manifest paths.
MSIX_VARIANT_ARTWORK = {
    "PackageLogo.scale-200.png": (
        "Properties/Logo", 100, 11343, "3cc4b770643c26b3eafe022f16636056c118bd255e23aab53d5241d673a1ef60",
    ),
    "PackageLogo.scale-400.png": (
        "Properties/Logo", 200, 32639, "08eab8242e9237ec077d281a666269338b00f79f0ec50c9ca5a905a8a0e6bad9",
    ),
    "Square150x150Logo.scale-200.png": (
        "VisualElements/@Square150x150Logo", 300, 61221, "346ab8b79a7f0f2ec77a31953a463e7079a990ccff98712a137e67f0d03d312d",
    ),
    "Square150x150Logo.scale-400.png": (
        "VisualElements/@Square150x150Logo", 600, 179490, "c19908b16111f79db51f115b684170cd9959df5ebd40cd315e4a0fa2e7219a7b",
    ),
    "Square44x44Logo.scale-200.png": (
        "VisualElements/@Square44x44Logo", 88, 11657, "004a305e513f5af9333491b3a003eaac982bd2a8827eb7b0f6e8467ecc86d83a",
    ),
    "Square44x44Logo.scale-400.png": (
        "VisualElements/@Square44x44Logo", 176, 36304, "0754d5073d245a294ea9c308a42a02ecc49a8c7901a7b74d0b2486d587db2e94",
    ),
    "Square44x44Logo.targetsize-16.png": (
        "VisualElements/@Square44x44Logo", 16, 692, "15fadf4e1d2a79437a903288f1195323dd04e3e68ec15f1cf3634d56a85ed0db",
    ),
    "Square44x44Logo.targetsize-16_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 16, 692, "15fadf4e1d2a79437a903288f1195323dd04e3e68ec15f1cf3634d56a85ed0db",
    ),
    "Square44x44Logo.targetsize-16_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 16, 823, "dbd4506dbea8832851e9ced3a585b0732f76a99866391be4f8d9ed99e67b57c0",
    ),
    "Square44x44Logo.targetsize-20.png": (
        "VisualElements/@Square44x44Logo", 20, 1253, "09deed9fba6850f0f33d617b5c7fe735a36e69730b9fded2bc3ad3cf3cdd2dfc",
    ),
    "Square44x44Logo.targetsize-20_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 20, 1253, "09deed9fba6850f0f33d617b5c7fe735a36e69730b9fded2bc3ad3cf3cdd2dfc",
    ),
    "Square44x44Logo.targetsize-20_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 20, 1391, "7db01213576a6b4e900e3b5474ba7c98e0b34ffe997ea15765194611ebb4ee67",
    ),
    "Square44x44Logo.targetsize-24.png": (
        "VisualElements/@Square44x44Logo", 24, 1164, "3a865759c141dd468b234ce647b580c4507ff0c0d1ef520a85012a4ef1d59816",
    ),
    "Square44x44Logo.targetsize-24_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 24, 1164, "3a865759c141dd468b234ce647b580c4507ff0c0d1ef520a85012a4ef1d59816",
    ),
    "Square44x44Logo.targetsize-24_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 24, 1423, "b2a481bbe1f7986d4878de8151f917d03bdeafde4ba6c4461c3c34abdf1ef9b8",
    ),
    "Square44x44Logo.targetsize-256.png": (
        "VisualElements/@Square44x44Logo", 256, 52768, "ead4e2eb86f1eb0bba15ca0ac26e436479eb5b1863859610d0cd2bf4b15a7673",
    ),
    "Square44x44Logo.targetsize-256_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 256, 52768, "ead4e2eb86f1eb0bba15ca0ac26e436479eb5b1863859610d0cd2bf4b15a7673",
    ),
    "Square44x44Logo.targetsize-256_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 256, 52768, "ead4e2eb86f1eb0bba15ca0ac26e436479eb5b1863859610d0cd2bf4b15a7673",
    ),
    "Square44x44Logo.targetsize-30.png": (
        "VisualElements/@Square44x44Logo", 30, 2227, "24b7552559572fa7cefbb536e1f28233c7b28bde68bedb09492186942745ba20",
    ),
    "Square44x44Logo.targetsize-30_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 30, 2227, "24b7552559572fa7cefbb536e1f28233c7b28bde68bedb09492186942745ba20",
    ),
    "Square44x44Logo.targetsize-30_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 30, 2525, "d26e14788a29ae05c6ec35314daeeac5dbd1ae2b1caffc8fcfc5188279b454e0",
    ),
    "Square44x44Logo.targetsize-32.png": (
        "VisualElements/@Square44x44Logo", 32, 1845, "a88ab8794adaef2a547180dd00769bab1e3dfc298daf48a779913dbed32157ac",
    ),
    "Square44x44Logo.targetsize-32_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 32, 1845, "a88ab8794adaef2a547180dd00769bab1e3dfc298daf48a779913dbed32157ac",
    ),
    "Square44x44Logo.targetsize-32_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 32, 2223, "dd91abcb6dd24500c12b2966303263136f398d2fc8e5bc095985dfd32975939d",
    ),
    "Square44x44Logo.targetsize-36.png": (
        "VisualElements/@Square44x44Logo", 36, 2797, "e6e335f2d5938e9cf2cbeb363b218e6457d508dca9f604c7a9575659d5c1bea2",
    ),
    "Square44x44Logo.targetsize-36_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 36, 2797, "e6e335f2d5938e9cf2cbeb363b218e6457d508dca9f604c7a9575659d5c1bea2",
    ),
    "Square44x44Logo.targetsize-36_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 36, 3250, "d9938cf5cdb32d344591a5af8f22a9a2eb186dfb3e081c33c439af02fd008e3e",
    ),
    "Square44x44Logo.targetsize-40.png": (
        "VisualElements/@Square44x44Logo", 40, 3187, "0db6ea746c22261d7a56efafd95f6a2b801693a8052913c0b1d822761e09ffb6",
    ),
    "Square44x44Logo.targetsize-40_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 40, 3187, "0db6ea746c22261d7a56efafd95f6a2b801693a8052913c0b1d822761e09ffb6",
    ),
    "Square44x44Logo.targetsize-40_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 40, 3752, "55e81205345682de158722e4608bcd824d2a6a725e1ff608aa3fcff54ec8bd55",
    ),
    "Square44x44Logo.targetsize-48.png": (
        "VisualElements/@Square44x44Logo", 48, 3139, "574623fed8f96e9a75b5822749da4726aa820ac14498a0c252f7458fe34a1aca",
    ),
    "Square44x44Logo.targetsize-48_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 48, 3139, "574623fed8f96e9a75b5822749da4726aa820ac14498a0c252f7458fe34a1aca",
    ),
    "Square44x44Logo.targetsize-48_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 48, 3922, "993fecdfb1f908e2a77fd65ae27a5960c0ed689f3c2abeec600cef4ac31ba8a1",
    ),
    "Square44x44Logo.targetsize-60.png": (
        "VisualElements/@Square44x44Logo", 60, 6586, "ddf8cb7d20600cf5e5a2a998a0c844ced8d46e8afac27cd823c4d73c08e5d939",
    ),
    "Square44x44Logo.targetsize-60_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 60, 6586, "ddf8cb7d20600cf5e5a2a998a0c844ced8d46e8afac27cd823c4d73c08e5d939",
    ),
    "Square44x44Logo.targetsize-60_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 60, 6716, "c3ec576d3a0f3edc840c21d36b7d88f71c847e3e410534809155be07539b0add",
    ),
    "Square44x44Logo.targetsize-64.png": (
        "VisualElements/@Square44x44Logo", 64, 5689, "118d2789ed9bffee67cc9421b9371f016c2b02268f600a6e04a920d23e7b64f0",
    ),
    "Square44x44Logo.targetsize-64_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 64, 5689, "118d2789ed9bffee67cc9421b9371f016c2b02268f600a6e04a920d23e7b64f0",
    ),
    "Square44x44Logo.targetsize-64_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 64, 6187, "a34ffb770d953a53514b8a7651ad614efd30319aeeede2a60a80432fbc8dfcf3",
    ),
    "Square44x44Logo.targetsize-72.png": (
        "VisualElements/@Square44x44Logo", 72, 8557, "b62bd2a0d88fb923da05698e8075c0c3c080e47f7c8537d08a77cf9bcd8a4f07",
    ),
    "Square44x44Logo.targetsize-72_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 72, 8557, "b62bd2a0d88fb923da05698e8075c0c3c080e47f7c8537d08a77cf9bcd8a4f07",
    ),
    "Square44x44Logo.targetsize-72_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 72, 8557, "b62bd2a0d88fb923da05698e8075c0c3c080e47f7c8537d08a77cf9bcd8a4f07",
    ),
    "Square44x44Logo.targetsize-80.png": (
        "VisualElements/@Square44x44Logo", 80, 9962, "21642b153551a285066c660e78985e98f73443332c1211be372ee0c01576a086",
    ),
    "Square44x44Logo.targetsize-80_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 80, 9962, "21642b153551a285066c660e78985e98f73443332c1211be372ee0c01576a086",
    ),
    "Square44x44Logo.targetsize-80_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 80, 9962, "21642b153551a285066c660e78985e98f73443332c1211be372ee0c01576a086",
    ),
    "Square44x44Logo.targetsize-96.png": (
        "VisualElements/@Square44x44Logo", 96, 13269, "904b35ea33dba2d64a5590a87bf47d6b440fe938821c6345295b6a95d7e652a6",
    ),
    "Square44x44Logo.targetsize-96_altform-lightunplated.png": (
        "VisualElements/@Square44x44Logo", 96, 13269, "904b35ea33dba2d64a5590a87bf47d6b440fe938821c6345295b6a95d7e652a6",
    ),
    "Square44x44Logo.targetsize-96_altform-unplated.png": (
        "VisualElements/@Square44x44Logo", 96, 13269, "904b35ea33dba2d64a5590a87bf47d6b440fe938821c6345295b6a95d7e652a6",
    ),
}
RC_COMPILER = "/tools/llvm/bin/llvm-rc"
RC_LIBRARY = "/tools/llvm/lib/libLLVM.so.22.1"
WINDOWS_UI_HOST_TOOLS = "lib/rustlib/x86_64-unknown-linux-gnu"
READOBJ = f"{WINDOWS_UI_HOST_TOOLS}/bin/llvm-readobj"
READOBJ_LIBRARY = f"{WINDOWS_UI_HOST_TOOLS}/lib/libLLVM.so.23.1-rust-1.100.0-stable"
LINKER_LLVM = f"/toolchain/lib/{Path(READOBJ_LIBRARY).name}"
RESOURCE_PATH = "/work/metadata/ui.res"
RESOURCE_DIGEST_ENV = "NEVROTICK_UI_VERSIONINFO_SHA256"

# Exact LLVM 22.1.8 resource tools and Rust 1.100.0's LLVM reader.
WINDOWS_UI_TOOL_PINS = {
    RC_COMPILER: (202112, "4bc604021aa8f408dfe212ae1de12b785e60d914f19e9ade02333d842135556e"),
    RC_LIBRARY: (149344344, "3f02d721ee01ce618a2317fe5552ab3bbd44331cd6d65e602dae2876fbf35e2e"),
    f"/toolchain/{READOBJ}": (3134320, "867e1f5d2a6d277d4ba5d01ba8205f301d1734ff6e7bb64663e2c8aaa2b35d63"),
    f"/toolchain/{READOBJ_LIBRARY}": (
        208242288, "517f2e2d3762ccd6fe742c1b2fc720f447777c2589c99f58e7d12bfacf7b908c"),
    LINKER_LLVM: (
        208242288, "517f2e2d3762ccd6fe742c1b2fc720f447777c2589c99f58e7d12bfacf7b908c"),
}


# Vendor-specific host-admission ceilings, before isolated build execution.
# The reviewed534-package graph has2.66MB checksum metadata (largest191KB),
# 31,563 tree entries and566MB source/metadata bytes. These are fixed ceilings,
# not values inferred from an untrusted vendor tree or its checksum records.
MAX_VENDOR_PACKAGES = 1024
MAX_VENDOR_METADATA_BYTES = 256 * 1024
MAX_VENDOR_METADATA_TOTAL = 8 * 1024 * 1024
MAX_VENDOR_ENTRIES = 65_536
MAX_VENDOR_PACKAGE_ENTRIES = 8192
MAX_VENDOR_DEPTH = 16
MAX_VENDOR_PATH_BYTES = 1024
MAX_VENDOR_FILE_BYTES = 64 * 1024 * 1024
MAX_VENDOR_TOTAL_BYTES = 1024 * 1024 * 1024
MAX_VENDOR_MAP_ENTRIES = 4096
MAX_VENDOR_JSON_TOKENS = 16_384

def sha256(path: Path) -> str:
    """Hash a regular input without loading its complete content."""
    if path.is_symlink() or not path.is_file():
        raise ENV.Incomplete(f"Expected regular admitted input: {path.name}")
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for data in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(data)
    return digest.hexdigest()


def inventory(directory: Path) -> dict[str, str]:
    """Capture regular tree bytes; symlinks and special files are not admitted."""
    if directory.is_symlink() or not directory.is_dir():
        raise ENV.Incomplete("Missing regular input directory.")
    result = {}
    for path in sorted(directory.rglob("*")):
        if path.is_symlink() or not (path.is_file() or path.is_dir()):
            raise ENV.Incomplete("Linked or special input is not admitted.")
        if path.is_file():
            result[str(path.relative_to(directory))] = sha256(path)
    return result


def read_dependency_policy(path: Path) -> dict:
    """Decode bounded policy JSON without duplicate fields or non-JSON numbers."""
    if path.is_symlink() or not path.is_file():
        raise ENV.Incomplete("Missing regular dependency admission.")
    with path.open("rb") as stream:
        payload = stream.read(SLINT.MAX_POLICY_BYTES + 1)
    if len(payload) > SLINT.MAX_POLICY_BYTES:
        raise ENV.Incomplete("Dependency admission exceeds its byte bound.")

    def unique_fields(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ENV.Incomplete("Duplicate dependency admission field.")
            result[key] = value
        return result

    def invalid_number(_value):
        raise ENV.Incomplete("Non-JSON dependency admission number.")

    try:
        policy = json.loads(payload.decode("utf-8"), object_pairs_hook=unique_fields, parse_constant=invalid_number)
    except (UnicodeError, json.JSONDecodeError, RecursionError) as error:
        raise ENV.Incomplete("Invalid dependency admission JSON.") from error
    if type(policy) is not dict:
        raise ENV.Incomplete("Dependency admission must be an object.")
    if policy.get("schema_version") != 3 and len(payload) > 512 * 1024:
        raise ENV.Incomplete("Legacy dependency admission exceeds its byte bound.")
    return policy


def validate_dependency_policy(policy: dict) -> None:
    """Admit exact selected/metadata roles and two bounded target feature rows."""
    if type(policy) is dict and policy.get("schema_version") == 3:
        try:
            SLINT.validate_policy(policy)
        except SLINT.Incomplete as error:
            raise ENV.Incomplete(str(error)) from error
        return
    required = {"schema_version", "scope", "lockfile_sha256", "manifests_sha256",
                "packages", "metadata_only_packages", "rows"}
    if (type(policy) is not dict or set(policy) != required or type(policy["schema_version"]) is not int
            or policy["schema_version"] != 2 or type(policy["scope"]) is not str
            or not 0 < len(policy["scope"]) <= 8192):
        raise ENV.Incomplete("Unadmitted dependency policy schema.")
    manifests = {"Cargo.toml", *(member + "/Cargo.toml" for member in LEGACY_PACKAGES.values())}
    if (type(policy["lockfile_sha256"]) is not str
            or not re.fullmatch(r"[0-9a-f]{64}", policy["lockfile_sha256"])
            or type(policy["manifests_sha256"]) is not dict
            or set(policy["manifests_sha256"]) != manifests
            or any(type(value) is not str or not re.fullmatch(r"[0-9a-f]{64}", value)
                   for value in policy["manifests_sha256"].values())):
        raise ENV.Incomplete("Dependency manifest/lock identities are malformed.")
    for field, versions in (("packages", REGISTRY_VERSIONS),
                            ("metadata_only_packages", METADATA_ONLY_VERSIONS)):
        records = policy[field]
        if type(records) is not list or len(records) != len(versions):
            raise ENV.Incomplete("The lock graph contains missing or unexpected packages.")
        keys = {"name", "version", "checksum", "license", "files_sha256"}
        keys |= {"selected_license", "build_script", "upstream_revision"} if field == "packages" else {"role"}
        identities = set()
        for entry in records:
            if type(entry) is not dict or set(entry) != keys:
                raise ENV.Incomplete("Unadmitted dependency record fields or role promotion.")
            name, version = entry["name"], entry["version"]
            if (type(name) is not str or type(version) is not str or versions.get(name) != version
                    or name in identities):
                raise ENV.Incomplete("The lock graph contains missing or unexpected packages.")
            identities.add(name)
            if (any(type(entry[key]) is not str or not re.fullmatch(r"[0-9a-f]{64}", entry[key])
                    for key in ("checksum", "files_sha256"))
                    or type(entry["license"]) is not str or not 0 < len(entry["license"]) <= 256):
                raise ENV.Incomplete("Dependency source/checksum admission is malformed.")
            if field == "metadata_only_packages":
                if entry["role"] != "metadata-only":
                    raise ENV.Incomplete("Unadmitted metadata-only dependency role.")
            else:
                grant = "BSL-1.0" if name == "ryu" else "MIT AND Unicode-3.0" if name == "unicode-ident" else "MIT"
                if (entry["selected_license"] != grant or type(entry["build_script"]) is not bool
                        or entry["build_script"] != (name + "@" + version in BUILD_SCRIPTS)
                        or type(entry["upstream_revision"]) is not str
                        or not re.fullmatch(r"[0-9a-f]{40}", entry["upstream_revision"])):
                    raise ENV.Incomplete("The product license/build-script admission needs a reviewed policy change.")
    if type(policy["rows"]) is not dict or set(policy["rows"]) != {HOST_TARGET, ENV.WINDOWS_TARGET}:
        raise ENV.Incomplete("Dependency target rows are missing or unexpected.")
    for target, row in policy["rows"].items():
        excluded = {"windows-link", "windows-sys"} if target == HOST_TARGET else {"libc"}
        expected = sorted(name + "@" + version for name, version in REGISTRY_VERSIONS.items() if name not in excluded)
        if (type(row) is not dict or set(row) != {"external_packages", "features", "allowed_build_scripts"}
                or row["external_packages"] != expected or type(row["features"]) is not dict
                or set(row["features"]) != set(expected)
                or row["allowed_build_scripts"] != sorted(BUILD_SCRIPTS & set(expected))):
            raise ENV.Incomplete("Dependency target package/build-script row differs from the reviewed graph.")
        for features in row["features"].values():
            if (type(features) is not list or len(features) > 256
                    or any(type(feature) is not str or not 0 < len(feature) <= 128 for feature in features)
                    or features != sorted(set(features))):
                raise ENV.Incomplete("Dependency target feature row is malformed.")


def validated_dependency_records(policy: dict, *, include_metadata: bool = False) -> list[dict]:
    """Return copied selected records, optionally including inert resolution sources."""
    validate_dependency_policy(policy)
    if type(include_metadata) is not bool:
        raise ENV.Incomplete("Dependency record role selection must be explicit.")
    records = policy["packages"] + (policy["metadata_only_packages"] if include_metadata else [])
    return [dict(record) for record in records]


def validate_dependency_metadata(target: str, output: str, policy: dict, *, profile: str = "default") -> dict:
    """Compare actual frozen Cargo target identities/features with admitted policy."""
    validate_dependency_policy(policy)
    if policy["schema_version"] == 3:
        try:
            return SLINT.validate_metadata(target, output, policy, profile=profile)
        except SLINT.Incomplete as error:
            raise ENV.Incomplete(str(error)) from error
    if target not in policy["rows"] or len(output.encode("utf-8")) > ENV.MAX_OUTPUT:
        raise ENV.Incomplete("Unadmitted or oversized dependency metadata row.")
    try:
        data = json.loads(output)
    except json.JSONDecodeError as error:
        raise ENV.Incomplete("Invalid dependency metadata JSON.") from error
    if (type(data) is not dict or type(data.get("packages")) is not list
            or len(data["packages"]) > 64 or type(data.get("resolve")) is not dict
            or type(data["resolve"].get("nodes")) is not list
            or len(data["resolve"]["nodes"]) > 64):
        raise ENV.Incomplete("Dependency metadata lacks its bounded resolved graph.")
    nodes = {}
    for node in data["resolve"]["nodes"]:
        if (type(node) is not dict or type(node.get("id")) is not str
                or node["id"] in nodes or type(node.get("features")) is not list):
            raise ENV.Incomplete("Dependency metadata has missing or duplicate nodes.")
        nodes[node["id"]] = node
    selected, locals_seen, package_ids = {}, set(), set()
    for package in data["packages"]:
        if (type(package) is not dict or type(package.get("id")) is not str
                or package["id"] in package_ids or package["id"] not in nodes):
            raise ENV.Incomplete("Dependency metadata package/node identities differ.")
        package_ids.add(package["id"])
        name, version = package.get("name"), package.get("version")
        if type(name) is not str or type(version) is not str or "source" not in package:
            raise ENV.Incomplete("Dependency metadata package identity is malformed.")
        if package.get("source") is None:
            if name not in LEGACY_PACKAGES or version != "0.0.0" or name in locals_seen:
                raise ENV.Incomplete("Dependency metadata contains an unreviewed local owner.")
            locals_seen.add(name)
        else:
            if (package["source"] != "registry+https://github.com/rust-lang/crates.io-index"
                    or type(name) is not str or type(version) is not str):
                raise ENV.Incomplete("Dependency metadata contains an unadmitted source.")
            identity = name + "@" + version
            if identity in selected:
                raise ENV.Incomplete("Dependency metadata contains duplicate package versions.")
            features = nodes[package["id"]]["features"]
            if (any(type(feature) is not str for feature in features)
                    or len(features) > 256 or len(set(features)) != len(features)):
                raise ENV.Incomplete("Dependency metadata features are malformed.")
            selected[identity] = sorted(features)
    if (package_ids != set(nodes) or locals_seen != set(LEGACY_PACKAGES)
            or selected != policy["rows"][target]["features"]):
        raise ENV.Incomplete("Actual dependency target packages/features differ from admission.")
    return {"target": target, "external_packages": sorted(selected), "features": selected}


def admit_patched_source(root: Path, record: dict) -> dict[str, str]:
    """Freeze only exact reviewed local derivatives and their preserved source rights."""
    try:
        SLINT.validate_path_patch(record)
    except SLINT.Incomplete as error:
        raise ENV.Incomplete(str(error)) from error
    if record["name"] in SLINT.EP1_PATCH_UPSTREAM:
        path = record["path"]
        directory = root / path
        if any(parent.is_symlink() for parent in (directory, *directory.parents) if parent != root.parent):
            raise ENV.Incomplete("Linked prototype-source ancestors are not admitted.")
        files = inventory(directory)
        digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        if (digest != record["files_sha256"]
                or any(files.get(name) != value for name, value in record["modified_files_sha256"].items())
                or any(files.get(name) != value for name, value in SLINT.EP1_PRESERVED_FILES[record["name"]].items())):
            raise ENV.Incomplete("Prototype patch source or preserved upstream rights changed.")
        manifest = tomllib.loads((directory / "Cargo.toml").read_text())
        if (manifest.get("package", {}).get("name") != record["name"]
                or manifest["package"].get("version") != record["version"]
                or manifest["package"].get("license") != record["license"]):
            raise ENV.Incomplete("Prototype patch manifest identity changed.")
        return {path + "/" + name: value for name, value in files.items()}
    is_spin = record["name"] == SLINT.PATCH_NAME
    path = record["path"]
    directory = root / path
    if any(path.is_symlink() for path in (root / "third-party", directory)):
        raise ENV.Incomplete("Linked patched-source ancestors are not admitted.")
    files = inventory(directory)
    if record["name"] == SLINT.SLINT_CORE_NAME:
        verify_slint_calendar_patch(directory, files, record)
        return {path + "/" + name: value for name, value in files.items()}
    preserved, modified = {
        SLINT.PATCH_NAME: (SLINT.PATCH_PRESERVED_FILES, SLINT.PATCH_MODIFIED_FILES),
        SLINT.ACCESSKIT_NAME: (SLINT.ACCESSKIT_PRESERVED_FILES, SLINT.ACCESSKIT_MODIFIED_FILES),
        SLINT.CONSUMER_NAME: (SLINT.CONSUMER_PRESERVED_FILES, SLINT.CONSUMER_MODIFIED_FILES),
    }[record["name"]]
    expected_files = (SLINT.PATCH_MODIFIED_FILES | set(preserved) | {".gitignore", "Makefile.toml", "README.md"}
                      if is_spin else modified | set(preserved))
    digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    if (set(files) != expected_files or digest != record["files_sha256"]
            or any(files.get(name) != value for name, value in record["modified_files_sha256"].items())
            or any(files.get(name) != value for name, value in preserved.items())):
        raise ENV.Incomplete("Patched source bytes, exact file set or preserved upstream rights changed.")
    manifest = tomllib.loads((directory / "Cargo.toml").read_text())
    if not is_spin:
        upstream = tomllib.loads((directory / "Cargo.toml.upstream").read_text())
        upstream["package"]["license"] = record["license"]
        if record["name"] == SLINT.ACCESSKIT_NAME:
            # The checked HWND owner-thread guard uses this existing dependency
            # feature only; every other manifest field retains its upstream value.
            features = upstream["dependencies"]["windows"]["features"]
            upstream["dependencies"]["windows"]["features"] = sorted([*features, "Win32_System_Threading"])
        if manifest != upstream:
            raise ENV.Incomplete("The AccessKit manifest may only change its exact composite source licence and reviewed owner-thread feature.")
    else:
        if (set(manifest) != {"package"} or manifest["package"].get("name") != SLINT.PATCH_NAME
                or manifest["package"].get("version") != "0.1.1"
                or manifest["package"].get("license") != SLINT.PATCH_UPSTREAM["license"]
                or "build" in manifest["package"] or "links" in manifest["package"]
                or "license-file" in manifest["package"]):
            raise ENV.Incomplete("The reviewed path patch must remain a dependency/feature/build-free library.")
        if tomllib.loads((directory / "Cargo.lock").read_text()) != {
            "version": 4, "package": [{"name": SLINT.PATCH_NAME, "version": "0.1.1"}]
        }:
            raise ENV.Incomplete("Standalone path-patch lock must contain only its real local identity.")
    return {path + "/" + name: value for name, value in files.items()}


def verify_slint_calendar_patch(directory: Path, files: dict[str, str], record: dict) -> None:
    """Admit only the calendar and licence changes to the exact upstream archive."""
    original = dict(files)
    original_manifest = original.pop("Cargo.toml.upstream", None)
    patch_note = original.pop("NEVROTICK-PATCH.md", None)
    if original_manifest is None or patch_note is None or "date_time.rs" not in original:
        raise ENV.Incomplete("The calendar derivative lacks its preserved manifest, source or provenance.")
    original["Cargo.toml"] = original_manifest
    original["date_time.rs"] = SLINT.SLINT_CORE_ORIGINAL_DATE
    original_digest = hashlib.sha256(json.dumps(original, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    if (original_digest != SLINT.SLINT_CORE_UPSTREAM["upstream_files_sha256"]
            or digest != record["files_sha256"]
            or files["date_time.rs"] == SLINT.SLINT_CORE_ORIGINAL_DATE
            or files.get(SLINT.SLINT_CORE_DEBUGGER_FILE) != SLINT.SLINT_CORE_DEBUGGER_SHA256
            or any(files.get(name) != value for name, value in record["modified_files_sha256"].items())):
        raise ENV.Incomplete("The calendar derivative differs from its exact permitted source changes.")
    expected = tomllib.loads((directory / "Cargo.toml.upstream").read_text())
    expected["package"]["license"] = "GPL-3.0-only"
    del expected["dependencies"]["chrono"]
    expected["dependencies"]["time"] = {"version": "=0.3.55", "default-features": False,
                                        "features": ["formatting", "large-dates", "parsing"]}
    expected["features"]["std"] = [
        "time/local-offset" if value == "chrono/std" else value for value in expected["features"]["std"]
        if value not in {"chrono/wasmbind", "chrono/clock"}
    ]
    manifest = tomllib.loads((directory / "Cargo.toml").read_text())
    if manifest != expected:
        raise ENV.Incomplete("The Slint calendar may only replace Chrono with the exact time dependency and GPL grant.")


def admit_first_party_dependency(root: Path, record: dict, workspace_lints: dict) -> dict[str, str]:
    """Freeze the AES nonmember's reviewed graph and standalone lint policy."""
    directory = root / record["path"]
    if any(parent.is_symlink() for parent in (directory, *directory.parents) if parent != root.parent):
        raise ENV.Incomplete("Linked nonmember-source ancestors are not admitted.")
    files = inventory(directory)
    digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    if digest != record["files_sha256"] or files.get("Cargo.toml") != record["manifest_sha256"]:
        raise ENV.Incomplete("The first-party nonmember source or manifest differs from admission.")
    manifest = tomllib.loads((directory / "Cargo.toml").read_text())
    expected_lints = {**workspace_lints, "rust": {**workspace_lints["rust"], "unsafe_code": "forbid"}}
    if manifest.get("lints") != expected_lints:
        raise ENV.Incomplete("The AES nonmember requires the strict lint tables with unsafe code forbidden.")
    expected_dependencies = {
        "aes-siv": {"version": "=0.8.0", "default-features": False, "features": ["zeroize"]},
        "aes": {"version": "=0.9.3", "default-features": False, "features": ["zeroize"]},
        "cmac": {"version": "=0.8.0", "default-features": False, "features": ["zeroize"]},
        "ntp-wire": {"version": "=0.0.0", "path": "../../crates/ntp-wire"},
        "zeroize": {"version": "=1.9.1", "default-features": False},
    }
    if (set(manifest) != {"package", "workspace", "lib", "dependencies", "lints"}
            or manifest["package"] != {"name": SLINT.EP1_AES_NAME, "version": "0.0.0", "edition": "2024",
                "rust-version": "1.99", "license": "GPL-2.0-only", "publish": False, "build": False}
            or manifest["workspace"] != {"resolver": "3"} or manifest["lib"] != {"path": "lib.rs"}
            or manifest["dependencies"] != expected_dependencies or "lib.rs" not in files):
        raise ENV.Incomplete("The AES nonmember requires only its exact reviewed pure library graph.")
    return {record["path"] + "/" + name: value for name, value in files.items()}


def validate_patched_lock(packages: list[dict], patches: list[dict]) -> None:
    """Bind local substitution to exact lock identities and unchanged normal edges."""
    try:
        SLINT.patch_names({"patched_packages": patches})
        for record in patches:
            SLINT.validate_path_patch(record)
    except SLINT.Incomplete as error:
        raise ENV.Incomplete(str(error)) from error
    expected = {SLINT.PATCH_NAME: {"name": SLINT.PATCH_NAME, "version": "0.1.1"},
                SLINT.ACCESSKIT_NAME: {"name": SLINT.ACCESSKIT_NAME, "version": "0.34.0",
                    "dependencies": ["accesskit", "accesskit_consumer", "hashbrown 0.16.1",
                                     "static_assertions", "windows", "windows-core"]},
                SLINT.CONSUMER_NAME: {"name": SLINT.CONSUMER_NAME, "version": "0.38.0",
                    "dependencies": ["accesskit", "hashbrown 0.16.1"]}}
    if any(record["name"] == SLINT.SLINT_CORE_NAME for record in patches):
        expected[SLINT.SLINT_CORE_NAME] = {
            "name": SLINT.SLINT_CORE_NAME, "version": "1.18.1",
            "dependencies": [
                "accesskit", "bitflags 2.13.2", "cfg-if", "clru", "const-field-offset", "derive_more",
                "euclid", "i-slint-common", "i-slint-core-macros", "icu_normalizer", "image", "js-sys",
                "lyon_algorithms", "lyon_extra", "lyon_geom", "lyon_path", "num-traits", "objc2 0.6.5",
                "objc2-app-kit 0.3.2", "objc2-foundation 0.3.2", "once_cell", "parley", "pin-project",
                "pin-weak", "portable-atomic", "raw-window-handle", "resvg", "rgb", "scopeguard",
                "skrifa", "slab", "strum", "swash", "sys-locale", "taffy", "time",
                "unicode-linebreak", "unicode-script", "unicode-segmentation", "vtable", "wasm-bindgen",
                "web-sys", "web-time", "wgpu", "windows 0.62.2",
            ],
        }
    if any(record["name"] in SLINT.EP1_PATCH_NAMES for record in patches):
        expected.update({
            "rustls-cng-crypto": {"name": "rustls-cng-crypto", "version": "0.1.2",
                "dependencies": ["once_cell", "pkcs1", "pkcs8", "rustls", "sec1", "windows 0.61.3", "zeroize"]},
            "rustls-platform-verifier": {"name": "rustls-platform-verifier", "version": "0.7.1",
                "dependencies": ["core-foundation 0.10.1", "core-foundation-sys", "jni", "log", "once_cell",
                    "rustls", "rustls-native-certs", "rustls-platform-verifier-android", "rustls-webpki",
                    "security-framework", "security-framework-sys", "webpki-root-certs", "windows-sys 0.61.2"]},
        })
        expected[SLINT.ACCESSKIT_NAME]["dependencies"] = [
            "windows 0.62.2" if edge == "windows" else "windows-core 0.62.2" if edge == "windows-core" else edge
            for edge in expected[SLINT.ACCESSKIT_NAME]["dependencies"]]
        counts = {name: sum(package["name"] == name for package in packages)
                  for name in {package["name"] for package in packages}}
        for value in expected.values():
            value["dependencies"] = sorted(edge if counts.get(edge.split(" ")[0], 0) > 1 else edge.split(" ")[0]
                                           for edge in value.get("dependencies", []))
            if not value["dependencies"]:
                del value["dependencies"]
    names = {record["name"] for record in patches}
    actual = [package for package in packages if package["name"] in names]
    if (len(actual) != len(names) or {package["name"] for package in actual} != names
            or any(package != expected[package["name"]] for package in actual)):
        raise ENV.Incomplete("A local path patch acquired a checksum, origin or unreviewed dependency edge.")


def require_root_cooldown_admission(root: Path, policy: dict) -> dict:
    """Require the reviewed current graph before acquisition or execution.

    Archive-only legacy controls isolate this boundary explicitly. They do not
    supply age approval for a product or imported registry graph.
    """
    try:
        configuration = ENV.require_cargo_context(root)
        if policy.get("schema_version") != 3:
            raise ENV.Incomplete("Current product acquisition requires its schema-3 cooldown admission.")
        if read_dependency_policy(root / "policy/ui-m0-dependencies.json") != policy:
            raise ENV.Incomplete("Dependency policy changed before graph admission.")
        validate_dependency_policy(policy)
        decision = SLINT.COOLDOWN.validate_root_admission(root, policy)
        return {"configuration": configuration, "decision_sha256": SLINT.COOLDOWN.canonical(decision)}
    except (OSError, ValueError, KeyError, TypeError) as error:
        raise ENV.Incomplete("Registry graph cooldown admission failed: " + str(error)) from error


def admit_product(root: Path) -> tuple[dict, dict[str, str]]:
    """Admit reviewed members, explicit engine effects and the exact locked graph."""
    try:
        STRICT.read_configuration(root)
    except STRICT.ENV.Incomplete as error:
        raise ENV.Incomplete(str(error)) from error
    policy = read_dependency_policy(root / "policy/ui-m0-dependencies.json")
    native_adapter = policy.get("schema_version") == 3 and SLINT.NATIVE_PACKAGE in SLINT.policy_members(policy)
    product_packages = PACKAGES if native_adapter else LEGACY_PACKAGES
    for member_path in (root / member for member in product_packages.values()):
        if (not member_path.resolve().is_relative_to(root.resolve())
                or any(path.is_symlink() for path in (root / "crates", member_path))):
            raise ENV.Incomplete("Linked product ancestors are not admitted.")
    for name in SOURCE_FILES:
        sha256(root / name)
    ENV.require_cargo_configuration(tomllib.loads((root / ".cargo/config.toml").read_text()))
    workspace = tomllib.loads((root / "Cargo.toml").read_text())
    fragment = tomllib.loads((root / "policy/rust-workspace.toml").read_text())
    if workspace.get("workspace", {}).get("members") != list(product_packages.values()):
        raise ENV.Incomplete("Expected exactly the reviewed UI-M0, wire, domain, exchange, core, engine and platform workspace members.")
    validate_dependency_policy(policy)
    slint_profile = policy["schema_version"] == 3
    patches = policy.get("patched_packages", [])
    expected_patch = {"crates-io": {record["name"]: {"path": record["path"]} for record in patches}} if patches else {}
    expected_excludes = [record["path"] for record in patches]
    if SLINT.EP1_FIELD in policy:
        expected_excludes.append(SLINT.EP1_AES_PATH)
    if (workspace.get("patch", {}) != expected_patch or "replace" in workspace
            or workspace["workspace"].get("exclude", []) != expected_excludes):
        raise ENV.Incomplete("Unadmitted patch routing, replacement or workspace exclusion.")
    workspace_package = workspace["workspace"].get("package", {})
    if (workspace_package.get("license") != "GPL-2.0-only"
            or "license-file" in workspace_package):
        raise ENV.Incomplete("The shared workspace license must remain GPL-2.0-only.")
    if (workspace["workspace"].get("lints") != fragment["workspace"]["lints"]
            or workspace.get("profile") != fragment["profile"]):
        raise ENV.Incomplete("Workspace lint/profile policy differs from its owner.")
    local_packages = set()
    for package, directory in product_packages.items():
        member = tomllib.loads((root / directory / "Cargo.toml").read_text())
        if (member.get("package", {}).get("name") != package
                or member.get("lints") != {"workspace": True}):
            raise ENV.Incomplete("A member identity or inherited lints are missing.")
        license_declaration = member["package"].get("license")
        if package == PACKAGE:
            license_admitted = license_declaration == "GPL-3.0-only"
        else:
            license_admitted = (license_declaration == {"workspace": True}
                                and type(license_declaration.get("workspace")) is bool)
        if not license_admitted or "license-file" in member["package"]:
            raise ENV.Incomplete("First-party license roles require explicit UI GPL-3.0-only and inherited GPL-2.0-only elsewhere.")
        if package == PACKAGE and slint_profile:
            if (member["package"].get("build") not in (None, "build.rs")
                    or sha256(root / SLINT.UI_BUILD) != policy["first_party_build_scripts"][SLINT.UI_BUILD]):
                raise ENV.Incomplete("The UI build script differs from its exact reviewed source.")
        elif member["package"].get("build") not in (None, False) or (root / directory / "build.rs").exists():
            raise ENV.Incomplete("An unadmitted first-party build script is present.")
        if package == PACKAGE:
            tables = [member, *member.get("target", {}).values()]
            for table in tables:
                for kind in ("dependencies", "dev-dependencies", "build-dependencies"):
                    for name, declaration in table.get(kind, {}).items():
                        if isinstance(declaration, dict) and "workspace" in declaration:
                            raise ENV.Incomplete("UI dependency inheritance is not admitted; runtime dependencies belong only to the engine.")
                        identity = declaration.get("package", name) if isinstance(declaration, dict) else name
                        if identity in {"tokio", "tokio-macros", "getrandom", "nevrotick-engine"}:
                            raise ENV.Incomplete("Runtime and transport dependencies belong only to the engine.")
            required = {"garde": {"version": "=0.23.0", "default-features": False, "features": ["derive"]}}
            required_target = {"cfg(windows)": {"dependencies": {
                "windows-sys": {"version": "=0.61.2", "default-features": False, "features": [
                    "Win32_Foundation", "Win32_Graphics_Gdi", "Win32_System_LibraryLoader",
                    "Win32_UI_WindowsAndMessaging", "Win32_UI_Controls",
                ]},
            }}}
            required_keys = {"package", "lints", "dependencies", "target"}
            if slint_profile:
                required.update({
                    "serde": {"version": "=1.0.229", "default-features": False, "features": ["std"]},
                    "serde_json": {"version": "=1.0.154", "default-features": False, "features": ["std"]},
                })
                required_target["cfg(windows)"]["dependencies"]["windows-sys"]["features"].append("Win32_UI_Shell")
                required_target["cfg(windows)"]["dependencies"]["windows"] = {
                    "version": "=0.62.2", "default-features": False,
                    "features": ["ApplicationModel", "Data_Xml_Dom", "Foundation", "UI_Notifications",
                                 "Win32_System_Com", "Win32_System_WinRT"]}
                required_target["cfg(windows)"]["dependencies"]["slint"] = {
                    "version": "=1.18.1", "default-features": False,
                    "features": ["std", "backend-winit", "renderer-software", "accessibility", "compat-1-18", "unstable-winit-030"]}
                required_target["cfg(not(windows))"] = {"dependencies": {"slint": {
                    "version": "=1.18.1", "default-features": False,
                    "features": ["std", "renderer-software", "compat-1-18"]},
                    "i-slint-core": {"version": "=1.18.1", "default-features": False,
                                     "features": ["image-decoders", "svg"]}}}
                required_keys.add("build-dependencies")
                if (member.get("build-dependencies") != {"slint-build": {"version": "=1.18.1", "default-features": False,
                                                                          "features": ["compat-1-18"]}}
                        or type(member["build-dependencies"]["slint-build"].get("default-features")) is not bool
                        or any(type(member.get("target", {}).get(condition, {}).get("dependencies", {}).get("slint", {}).get("default-features")) is not bool
                               for condition in ("cfg(windows)", "cfg(not(windows))"))):
                    raise ENV.Incomplete("The UI requires its exact reviewed Slint runtime/build profile.")
            if (set(member) != required_keys
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or member.get("target") != required_target
                    or any(type(member["dependencies"][name].get("default-features")) is not bool
                           for name in required)
                    or type(member["target"]["cfg(windows)"]["dependencies"]["windows-sys"].get("default-features")) is not bool
                    or slint_profile and type(member["target"]["cfg(windows)"]["dependencies"]["windows"].get("default-features")) is not bool):
                raise ENV.Incomplete("The UI requires only its exact reviewed baseline dependencies, defaults and target features.")
        if package == "ntp-wire" and set(member) != {"package", "lints"}:
            raise ENV.Incomplete("The pure NTP-wire member may not add dependencies, features or targets.")
        if package == SLINT.NATIVE_PACKAGE and (
                set(member) != {"package", "lints"} or member["package"].get("version") != "0.0.0"
                or "links" in member["package"]):
            raise ENV.Incomplete("The native platform adapter requires its dependency-free manifest without features or execution roles.")
        if package in {"time-domain", "time-core", "windows-platform"}:
            required = {"garde": {"version": "=0.23.0", "default-features": False, "features": ["derive"]}}
            if package == "time-core":
                required["time-domain"] = {"version": "=0.0.0", "path": "../time-domain"}
            if (set(member) != {"package", "lints", "dependencies"}
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or type(member.get("dependencies", {}).get("garde", {}).get("default-features")) is not bool):
                if package == "time-domain":
                    raise ENV.Incomplete("The time-domain member requires only exact garde with the reviewed features.")
                if package == "windows-platform":
                    raise ENV.Incomplete("The windows-platform member requires only exact garde with the reviewed features.")
                raise ENV.Incomplete("The time-core member requires its exact reviewed garde/local dependency graph.")
        if package == "ntp-exchange":
            required = {"ntp-wire": {"version": "=0.0.0", "path": "../ntp-wire"},
                        "time-domain": {"version": "=0.0.0", "path": "../time-domain"}}
            required_dev = {"time-core": {"version": "=0.0.0", "path": "../time-core"}}
            if (set(member) != {"package", "lints", "dependencies", "dev-dependencies"}
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or member.get("dev-dependencies") != required_dev):
                raise ENV.Incomplete("The ntp-exchange member requires only exact local ntp-wire and time-domain normal dependencies and the time-core dev dependency.")
        if package == "nevrotick-engine":
            required = {
                "garde": {"version": "=0.23.0", "default-features": False, "features": ["derive"]},
                "getrandom": {"version": "=0.4.3", "default-features": False},
                "ntp-wire": {"version": "=0.0.0", "path": "../ntp-wire"},
                "time-domain": {"version": "=0.0.0", "path": "../time-domain"},
                "tokio": {"version": "=1.54.1", "default-features": False,
                          "features": ["rt", "net", "time", "sync", "macros"]},
            }
            required_keys = {"package", "lints", "dependencies", "bin"}
            expected_target = {"cfg(windows)": {"dependencies": {
                SLINT.NATIVE_PACKAGE: {"version": "=0.0.0", "path": "../" + SLINT.NATIVE_PACKAGE},
            }}} if native_adapter else None
            if native_adapter:
                required_keys.add("target")
            if SLINT.EP1_FIELD in policy:
                required.update({
                    SLINT.EP1_AES_NAME: {"version": "=0.0.0", "path": "../../tools/crypto-aes-siv-witness",
                        "default-features": False, "optional": True},
                    "rustls-pki-types": {"version": "=1.15.1", "default-features": False,
                        "features": ["std"], "optional": True},
                    "zeroize": {"version": "=1.9.1", "default-features": False,
                        "features": ["alloc"], "optional": True},
                })
                if expected_target is None:
                    expected_target = {"cfg(windows)": {"dependencies": {}}}
                expected_target["cfg(windows)"]["dependencies"].update({
                    "rustls": {"version": "=0.23.46", "default-features": False, "features": ["std"], "optional": True},
                    "rustls-cng-crypto": {"version": "=0.1.2", "default-features": False, "optional": True},
                    "rustls-platform-verifier": {"version": "=0.7.1", "default-features": False, "optional": True},
                })
                expected_feature: dict[str, list[str]] = {SLINT.EP1_FEATURE: ["dep:" + name for name in
                    (SLINT.EP1_AES_NAME, "rustls-pki-types", "zeroize", "rustls", "rustls-cng-crypto", "rustls-platform-verifier")]}
                if SLINT.RGT_FIELD in policy:
                    required.update({name: {"version": "=" + version, "default-features": False, "optional": True}
                                     for name, version in RGT_ENGINE_VERSIONS.items()})
                    expected_feature[SLINT.RGT_FEATURE] = ["dep:" + name for name in RGT_ENGINE_VERSIONS]
                if (member.get("features") != expected_feature or member.get("target") != expected_target
                        or member.get("dependencies") != required):
                    raise ENV.Incomplete("The Engine requires its exact default-off features and reviewed provider graph.")
                normal_optional: tuple[str, ...] = (SLINT.EP1_AES_NAME, "rustls-pki-types", "zeroize")
                if SLINT.RGT_FIELD in policy:
                    normal_optional += tuple(RGT_ENGINE_VERSIONS)
                declarations: list[dict[str, object]] = [member["dependencies"][name] for name in normal_optional]
                declarations += [member["target"]["cfg(windows)"]["dependencies"][name]
                                 for name in ("rustls", "rustls-cng-crypto", "rustls-platform-verifier")]
                if any(type(item.get(field)) is not bool for item in declarations
                       for field in ("default-features", "optional")):
                    raise ENV.Incomplete("The Engine prototype requires boolean optional/default feature declarations.")
                required_keys |= {"features", "target"}
            if (set(member) != required_keys
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or member.get("target") != expected_target
                    or type(member["dependencies"]["garde"].get("default-features")) is not bool
                    or type(member["dependencies"]["getrandom"].get("default-features")) is not bool
                    or type(member["dependencies"]["tokio"].get("default-features")) is not bool
                    or member.get("bin") != [{"name": "nevrotick-observe", "path": "src/bin/nevrotick-observe.rs"}]):
                raise ENV.Incomplete("The engine requires its exact reviewed validation/runtime/wire/domain dependencies and observe entrypoint.")
        local_packages.add((package, member["package"]["version"]))
    dependency = SLINT.first_party_dependency(policy) if slint_profile else None
    if dependency:
        local_packages.add((dependency["name"], dependency["version"]))
    local_packages.update((record["name"], record["version"]) for record in patches)
    if policy.get("lockfile_sha256") != sha256(root / "Cargo.lock"):
        raise ENV.Incomplete("The product lockfile differs from its dependency admission.")
    manifests = {name: sha256(root / name)
                 for name in ("Cargo.toml", *(member + "/Cargo.toml" for member in product_packages.values()))}
    if policy.get("manifests_sha256") != manifests:
        raise ENV.Incomplete("Product manifest/features differ from the reviewed dependency admission.")
    require_root_cooldown_admission(root, policy)
    admitted = validated_dependency_records(policy, include_metadata=True)
    expected = {(p["name"], p["version"], p["checksum"]) for p in admitted}
    if not expected or len(expected) != len(admitted):
        raise ENV.Incomplete("Dependency admission must contain unique nonempty packages.")
    packages = tomllib.loads((root / "Cargo.lock").read_text()).get("package", [])
    # Cargo qualifies edges when the entire lock contains several versions,
    # including versions inactive in this package's selected feature scope.
    engine_random = ("getrandom 0.4.3" if sum(p["name"] == "getrandom" for p in packages) > 1
                     else "getrandom")
    exchange_core = ("time-core 0.0.0" if any(record["name"] == SLINT.SLINT_CORE_NAME for record in patches)
                     else "time-core")
    engine_edges = ["garde", engine_random, "ntp-wire", "time-domain", "tokio"]
    if native_adapter:
        engine_edges.append(SLINT.NATIVE_PACKAGE)
    if dependency:
        engine_edges = sorted([*engine_edges, SLINT.EP1_AES_NAME, "rustls", "rustls-cng-crypto",
                               "rustls-pki-types", "rustls-platform-verifier", "zeroize"])
    if SLINT.RGT_FIELD in policy:
        engine_edges = sorted([*engine_edges, *RGT_ENGINE_VERSIONS])
    external = {(p["name"], p["version"], p.get("checksum")) for p in packages if "source" in p}
    local = [p for p in packages if "source" not in p]
    if (external != expected or len([p for p in packages if "source" in p]) != len(expected)
            or len(local) != len(local_packages)
            or {(p["name"], p["version"]) for p in local} != local_packages
            or any(p.get("dependencies") for p in local if p["name"] in {"ntp-wire", SLINT.NATIVE_PACKAGE})
            or any(p.get("dependencies") != ["garde"] for p in local if p["name"] == "time-domain")
            or any(p.get("dependencies") != ["garde", "time-domain"] for p in local if p["name"] == "time-core")
            # Cargo.lock combines dependency kinds; the manifest guard above owns their roles.
            or any(p.get("dependencies") != ["ntp-wire", exchange_core, "time-domain"] for p in local if p["name"] == "ntp-exchange")
            or any(p.get("dependencies") != engine_edges for p in local if p["name"] == "nevrotick-engine")
            or not any(p["name"] == "garde" and p["version"] == "0.23.0" for p in packages if "source" in p)):
        raise ENV.Incomplete("The lock graph contains missing or unexpected packages.")
    if any(p.get("source") != "registry+https://github.com/rust-lang/crates.io-index" for p in packages if "source" in p):
        raise ENV.Incomplete("Unadmitted dependency origin.")
    if dependency:
        aes = [item for item in local if item["name"] == SLINT.EP1_AES_NAME]
        if aes != [{"name": SLINT.EP1_AES_NAME, "version": "0.0.0",
                    "dependencies": ["aes", "aes-siv", "cmac", "ntp-wire", "zeroize"]}]:
            raise ENV.Incomplete("The nonmember AES lock identity or normal edges changed.")
    validate_patched_lock(packages, patches)
    inputs = {name: sha256(root / name) for name in SOURCE_FILES}
    for record in patches:
        inputs.update(admit_patched_source(root, record))
    if dependency:
        inputs.update(admit_first_party_dependency(root, dependency, fragment["workspace"]["lints"]))
    for member in product_packages.values():
        for name, digest in inventory(root / member).items():
            if (name.endswith("Cargo.toml") and name != "Cargo.toml") or ".cargo" in Path(name).parts:
                raise ENV.Incomplete("Unexpected nested manifest or Cargo configuration.")
            inputs[f"{member}/{name}"] = digest
        if f"{member}/src/lib.rs" not in inputs:
            raise ENV.Incomplete("A required product library is absent.")
    for name in ("src/bin/nevrotick-observe.rs", "tests/runtime_boundary.rs"):
        if "crates/nevrotick-engine/" + name not in inputs:
            raise ENV.Incomplete("The required engine entrypoint or runtime-boundary suite is absent.")
    for name in (*STRICT.CONFIG_FILES, *CHECKER_FILES, "policy/ui-m0-dependencies.json", "policy/tool-manifest.json"):
        inputs[name] = sha256(root / name)
    if slint_profile:
        for name in (*UI_TEXT_CHECKER_FILES, *UI_NATIVE_INPUT_CHECKER_FILES):
            inputs[name] = sha256(root / name)
    return policy, inputs


def product_deny(base: str, policy: dict, target: str, *, profile: str = "default") -> str:
    """Add only this reviewed graph's license grants and exact build-script versions."""
    parsed = tomllib.loads(base)
    validate_dependency_policy(policy)
    if target not in policy["rows"]:
        raise ENV.Incomplete("Unadmitted dependency policy target.")
    if policy["schema_version"] == 3:
        if (parsed["licenses"]["allow"] != ["GPL-2.0-only"] or parsed["licenses"]["exceptions"]
                or parsed["bans"]["build"]["allow-build-scripts"] or parsed["bans"]["skip"]
                or "bypass" in parsed["bans"]["build"]):
            raise ENV.Incomplete("Bootstrap dependency policy was unexpectedly broadened.")
        records = SLINT.selected_records(policy)
        row = SLINT.profile_rows(policy, profile)[target]
        exceptions = [{"crate": key, "allow": records[key]["selected_license"].split(" AND ")}
                      for key in sorted(row["features"])]
        exceptions.append({"crate": "nevrotick-ui@0.0.0", "allow": ["GPL-3.0-only"]})
        duplicates = [{
            "crate": key,
            "reason": ("Distinct NevroTick estimator and time calendar support crates with exact source identities."
                       if key in SLINT.SLINT_CORE_DUPLICATE_IDS
                       else "Reviewed exact Slint software-profile duplicate; source identity pinned."),
        } for key in row["duplicate_exceptions"]]
        exception_text = "[" + ", ".join("{ crate = " + json.dumps(item["crate"]) + ", allow = "
                                           + json.dumps(item["allow"]) + " }" for item in exceptions) + "]"
        duplicate_text = "[" + ", ".join("{ crate = " + json.dumps(item["crate"]) + ", reason = "
                                           + json.dumps(item["reason"]) + " }" for item in duplicates) + "]"
        replacements = {"exceptions = []": "exceptions = " + exception_text,
                        "allow-build-scripts = []": "allow-build-scripts = " + json.dumps(row["allowed_build_scripts"]),
                        "skip = []": "skip = " + duplicate_text}
        result = base
        for before, after in replacements.items():
            if base.count(before) != 1:
                raise ENV.Incomplete("Dependency policy generation anchors changed.")
            result = result.replace(before, after, 1)
        parsed["licenses"]["exceptions"] = exceptions
        parsed["bans"]["build"]["allow-build-scripts"] = row["allowed_build_scripts"]
        parsed["bans"]["skip"] = duplicates
        bypasses = {}
        for entry in policy.get("archive_file_bypasses", []):
            if entry["package"] in row["features"]:
                bypasses.setdefault(entry["package"], []).append({"path": entry["path"], "checksum": entry["sha256"]})
        if bypasses:
            generated = []
            for key, paths in sorted(bypasses.items()):
                paths.sort(key=lambda item: item["path"])
                generated.append({"crate": key, "allow": paths})
                result += "\n[[bans.build.bypass]]\ncrate = " + json.dumps(key) + "\nallow = [" + ", ".join(
                    "{ path = " + json.dumps(item["path"]) + ", checksum = " + json.dumps(item["checksum"]) + " }"
                    for item in paths) + "]\n"
            # No build-script/required-features/allow-globs shortcut: those may
            # bypass the complete package scan rather than this exact file.
            parsed["bans"]["build"]["bypass"] = generated
        if tomllib.loads(result) != parsed:
            raise ENV.Incomplete("Product policy generation changed an unrelated dependency rule.")
        return result
    expressions = {entry["selected_license"] for entry in policy["packages"]}
    if expressions != {"BSL-1.0", "MIT", "MIT AND Unicode-3.0"}:
        raise ENV.Incomplete("The product license/build-script admission needs a reviewed policy change.")
    # Preserve the complete conjunctive unicode-ident grant; neither operand
    # alone is this package's selected expression.
    licenses = ["BSL-1.0", "MIT", "Unicode-3.0"]
    # cargo-deny 0.20.2 PackageSpec uses '@' for an exact version. '=' alone
    # remains part of the crate name and would reject the reviewed scripts.
    builds = policy["rows"][target]["allowed_build_scripts"]
    if (parsed["licenses"]["allow"] != ["GPL-2.0-only"] or parsed["licenses"]["exceptions"]
            or parsed["bans"]["build"]["allow-build-scripts"]):
        raise ENV.Incomplete("Bootstrap dependency policy was unexpectedly broadened.")
    before = 'allow = ["GPL-2.0-only"]'
    build_before = "allow-build-scripts = []"
    exception_before = "exceptions = []"
    if base.count(before) != 1 or base.count(build_before) != 1 or base.count(exception_before) != 1:
        raise ENV.Incomplete("Dependency policy generation anchors changed.")
    exceptions = [{"crate": "nevrotick-ui@0.0.0", "allow": ["GPL-3.0-only"]}]
    result = base.replace(before, "allow = " + json.dumps(["GPL-2.0-only", *licenses]), 1)
    result = result.replace(build_before, "allow-build-scripts = " + json.dumps(builds), 1)
    result = result.replace(exception_before,
                            'exceptions = [{ crate = "nevrotick-ui@0.0.0", allow = ["GPL-3.0-only"] }]', 1)
    parsed["licenses"]["allow"] = ["GPL-2.0-only", *licenses]
    parsed["licenses"]["exceptions"] = exceptions
    parsed["bans"]["build"]["allow-build-scripts"] = builds
    if tomllib.loads(result) != parsed:
        raise ENV.Incomplete("Product policy generation changed an unrelated dependency rule.")
    return result


def product_nextest(base: str) -> str:
    """Put nextest's separate store in owned output without changing test policy."""
    expected = tomllib.loads(base)
    if "store" in expected:
        raise ENV.Incomplete("Unexpected nextest store override in the admitted base policy.")
    # nextest's store is relative to the workspace, independently of Cargo's
    # CARGO_TARGET_DIR. An absolute store keeps the source snapshot read-only.
    result = base + '\n[store]\ndir = "/work/target/nextest"\n'
    expected["store"] = {"dir": "/work/target/nextest"}
    if tomllib.loads(result) != expected:
        raise ENV.Incomplete("Nextest storage configuration changed test policy.")
    return result


def vendor_scan(directory: Path, wanted: set[str]) -> dict[str, os.stat_result]:
    """Bound the entire vendor namespace and declared bytes before content I/O."""
    if directory.is_symlink() or not directory.is_dir():
        raise ENV.Incomplete("Missing regular input directory.")
    files, packages, stack = {}, set(), []
    entries = total_bytes = metadata_bytes = 0
    with os.scandir(directory) as children:
        for child in children:
            entries += 1
            if (entries > MAX_VENDOR_ENTRIES or len(packages) >= MAX_VENDOR_PACKAGES
                    or child.name not in wanted):
                raise ENV.Incomplete("Vendor directory has missing or unexpected packages.")
            if not child.is_dir(follow_symlinks=False):
                raise ENV.Incomplete("Linked or special input is not admitted.")
            packages.add(child.name)
            stack.append((Path(child.path), child.name, 0))
    if packages != wanted:
        raise ENV.Incomplete("Vendor directory has missing or unexpected packages.")
    package_entries = dict.fromkeys(packages, 0)
    while stack:
        folder, package, depth = stack.pop()
        with os.scandir(folder) as children:
            for child in children:
                entries += 1
                package_entries[package] += 1
                if (entries > MAX_VENDOR_ENTRIES
                        or package_entries[package] > MAX_VENDOR_PACKAGE_ENTRIES):
                    raise ENV.Incomplete("Vendor traversal exceeds its entry bound.")
                if depth + 1 > MAX_VENDOR_DEPTH:
                    raise ENV.Incomplete("Vendor traversal exceeds its depth bound.")
                path = Path(child.path)
                relative = path.relative_to(directory / package).as_posix()
                if len(relative.encode("utf-8")) > MAX_VENDOR_PATH_BYTES:
                    raise ENV.Incomplete("Vendor relative path exceeds its byte bound.")
                observed = child.stat(follow_symlinks=False)
                if stat.S_ISDIR(observed.st_mode):
                    stack.append((path, package, depth + 1))
                elif stat.S_ISREG(observed.st_mode):
                    total_bytes += observed.st_size
                    if observed.st_size > MAX_VENDOR_FILE_BYTES or total_bytes > MAX_VENDOR_TOTAL_BYTES:
                        raise ENV.Incomplete("Vendor source exceeds its file or aggregate byte bound.")
                    if relative == ".cargo-checksum.json":
                        metadata_bytes += observed.st_size
                        if observed.st_size > MAX_VENDOR_METADATA_BYTES:
                            raise ENV.Incomplete("Vendor checksum metadata exceeds its byte bound.")
                        if metadata_bytes > MAX_VENDOR_METADATA_TOTAL:
                            raise ENV.Incomplete("Vendor checksum metadata exceeds its aggregate byte bound.")
                    files[package + "/" + relative] = observed
                else:
                    raise ENV.Incomplete("Linked or special input is not admitted.")
    if any(package + "/.cargo-checksum.json" not in files for package in packages):
        raise ENV.Incomplete("Vendor package is missing its checksum metadata.")
    return files


def vendor_stamp(observed: os.stat_result) -> tuple[int, ...]:
    """Bind an opened regular input to the preflight inode and read bookends."""
    return (observed.st_dev, observed.st_ino, observed.st_mode, observed.st_size,
            observed.st_mtime_ns, observed.st_ctime_ns)


def vendor_file(path: Path, expected: os.stat_result, *, capture: bool) -> tuple[str, bytes]:
    """Hash at most the preflight byte reservation; retain only bounded metadata."""
    descriptor = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or vendor_stamp(before) != vendor_stamp(expected):
            raise ENV.Incomplete("Vendor input changed after bounded preflight.")
        remaining, digest, chunks = expected.st_size, hashlib.sha256(), []
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            while True:
                block = stream.read(min(1024 * 1024, remaining + 1))
                if not block:
                    break
                if len(block) > remaining:
                    raise ENV.Incomplete("Vendor input exceeded its reserved byte bound.")
                remaining -= len(block)
                digest.update(block)
                if capture:
                    chunks.append(block)
        if remaining or vendor_stamp(os.fstat(descriptor)) != vendor_stamp(before):
            raise ENV.Incomplete("Vendor input changed during bounded hashing.")
        return digest.hexdigest(), b"".join(chunks)
    finally:
        os.close(descriptor)


def vendor_inventory(directory: Path, wanted: set[str]) -> tuple[dict[str, str], dict[str, bytes]]:
    """Preflight aggregate work, then hash those exact files within reservations."""
    files = vendor_scan(directory, wanted)
    identities, metadata = {}, {}
    for relative, observed in files.items():
        capture = relative.endswith("/.cargo-checksum.json") and relative.count("/") == 1
        identities[relative], raw = vendor_file(directory / relative, observed, capture=capture)
        if capture:
            metadata[relative.split("/", 1)[0]] = raw
    return identities, metadata


def vendor_record(raw: bytes) -> dict:
    """Strictly admit the closed Cargo checksum shape under byte/depth/work bounds."""
    if type(raw) is not bytes or len(raw) > MAX_VENDOR_METADATA_BYTES:
        raise ENV.Incomplete("Vendor checksum metadata exceeds its byte bound.")
    depth = tokens = 0
    inside = escaped = False
    for byte in raw:
        if inside:
            if escaped:
                escaped = False
            elif byte == 92:
                escaped = True
            elif byte == 34:
                inside = False
        elif byte == 34:
            inside = True
            tokens += 1
        elif byte in (123, 91):
            depth += 1
            tokens += 1
            if depth > 2:
                raise ENV.Incomplete("Vendor checksum metadata exceeds its depth bound.")
        elif byte in (125, 93):
            depth -= 1
            if depth < 0:
                raise ENV.Incomplete("Malformed vendor checksum metadata nesting.")
        if tokens > MAX_VENDOR_JSON_TOKENS:
            raise ENV.Incomplete("Vendor checksum metadata exceeds its token bound.")
    if inside or depth:
        raise ENV.Incomplete("Malformed vendor checksum metadata nesting.")

    def unique_fields(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ENV.Incomplete("Duplicate vendor checksum metadata field.")
            result[key] = value
        return result

    def invalid_number(_value):
        raise ENV.Incomplete("Unselected vendor checksum metadata number.")

    try:
        record = json.loads(raw.decode("utf-8"), object_pairs_hook=unique_fields,
                            parse_int=invalid_number, parse_float=invalid_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise ENV.Incomplete("Invalid bounded vendor checksum JSON.") from error
    if (type(record) is not dict or set(record) != {"package", "files"}
            or type(record["package"]) is not str or not re.fullmatch(r"[0-9a-f]{64}", record["package"])
            or type(record["files"]) is not dict or not 0 < len(record["files"]) <= MAX_VENDOR_MAP_ENTRIES):
        raise ENV.Incomplete("Vendor checksum metadata has an unadmitted shape.")
    for relative, checksum in record["files"].items():
        try:
            encoded_path = relative.encode("utf-8")
        except UnicodeError as error:
            raise ENV.Incomplete("Vendor checksum file map has an unadmitted path or digest.") from error
        if (not relative or len(encoded_path) > MAX_VENDOR_PATH_BYTES or "\\" in relative
                or relative.startswith("/") or any(part in {"", ".", ".."} for part in relative.split("/"))
                or any(ord(character) < 32 for character in relative)
                or len(relative.split("/")) > MAX_VENDOR_DEPTH
                or type(checksum) is not str or not re.fullmatch(r"[0-9a-f]{64}", checksum)):
            raise ENV.Incomplete("Vendor checksum file map has an unadmitted path or digest.")
    return record


def admit_vendor(directory: Path, policy: dict) -> dict[str, str]:
    """Verify each vendored file against its reviewed package and Cargo checksum map."""
    records = policy["packages"] + policy.get("metadata_only_packages", [])
    wanted = {f"{p['name']}-{p['version']}": p for p in records}
    if len(wanted) != len(records):
        raise ENV.Incomplete("Vendor package roles are duplicated.")
    identities, metadata = vendor_inventory(directory, set(wanted))
    for name, admission in wanted.items():
        prefix = name + "/"
        files = {relative.removeprefix(prefix): checksum for relative, checksum in identities.items()
                 if relative.startswith(prefix)}
        record = vendor_record(metadata[name])
        if record.get("package") != admission["checksum"]:
            raise ENV.Incomplete("Vendor archive identity differs from the admitted lock graph.")
        files.pop(".cargo-checksum.json", None)
        if not files or files != record.get("files"):
            raise ENV.Incomplete("Vendor source bytes differ from their checksum inventory.")
        map_hash = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        if admission.get("files_sha256") != map_hash:
            raise ENV.Incomplete("Vendor checksum map differs from the reviewed archive extraction.")
    for bypass in policy.get("archive_file_bypasses", []):
        identity = bypass["package"]
        if identity == SLINT.SLINT_CORE_ID and any(
                record["name"] == SLINT.SLINT_CORE_NAME for record in policy.get("patched_packages", [])):
            if not SLINT.slint_core_debugger_bypass(bypass):
                raise ENV.Incomplete("The local calendar patch cannot broaden its preserved debugger file selection.")
            continue  # The local source owner admits these read-only bytes.
        matched = next((record for record in policy["packages"]
                        if record["name"] + "@" + record["version"] == identity), None)
        relative = bypass["path"]
        if (matched is None or not re.fullmatch(r"[A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*", relative)
                or any(part in {".", ".."} for part in relative.split("/"))
                or identities.get(matched["name"] + "-" + matched["version"] + "/" + relative) != bypass["sha256"]):
            raise ENV.Incomplete("An exact archive-file bypass does not match the admitted source bytes.")
    if vendor_inventory(directory, set(wanted))[0] != identities:
        raise ENV.Incomplete("Vendor inputs changed during admission.")
    return identities


def admit_tools(root: Path, toolchain: Path, extension: Path | None, lane: str) -> dict:
    """Hash selected tools before execution; qualify host isolation separately."""
    if sys.platform != "linux" or os.geteuid() == 0:
        raise ENV.Incomplete("Product verification requires an ordinary Linux/WSL user.")
    policy = json.loads((root / "policy/tool-manifest.json").read_text())
    pinned = tomllib.loads((root / "rust-toolchain.toml").read_text())["toolchain"]
    if pinned["channel"] != policy["rust"]["channel"]:
        raise ENV.Incomplete("Compiler pins disagree.")
    identities = {}
    for name in ("cargo", "rustc", "rustdoc", "rustfmt", "cargo-fmt", "cargo-clippy", "clippy-driver"):
        identities[name] = ENV.require_admitted_binary(toolchain / "bin" / name,
                                                      policy["rust"]["linux_binary_sha256"][name], name)
    if lane in {"portable", "dependencies"}:
        name = "cargo-nextest" if lane == "portable" else "cargo-deny"
        if extension is None:
            raise ENV.Incomplete(f"The pinned {name} is missing.")
        entry = next(p for p in policy["tools"] if p["name"] == name)
        identities[name] = ENV.require_admitted_binary(extension, entry["binary_sha256"], name)
    for name in ("bwrap", "prlimit", "cc", "ld", "python3"):
        found = shutil.which(name)
        if not found:
            raise ENV.Incomplete(f"Required host capability is missing: {name}")
        identities[name] = sha256(Path(found).resolve())
    return identities


def text_contract_commands(lane: str, *, schema_version: int) -> list[tuple[str, list[str]]]:
    """Check source-to-text coverage and its fault controls inside the frozen row."""
    if type(schema_version) is not int or schema_version not in (2, 3):
        raise ENV.Incomplete("Unknown text-contract verification schema.")
    if lane not in ("portable", "windows", "dependencies"):
        raise ENV.Incomplete("Unknown text-contract verification row.")
    if schema_version == 2:
        return []
    return [
        ("ui-text-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                              "discover", "-s", "scripts/tests", "-p", "test_ui_text_coverage.py", "-v"]),
        ("ui-text-codegen-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                      "discover", "-s", "scripts/tests", "-p", "test_ui_text_codegen.py", "-v"]),
        ("ui-text-native-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                      "discover", "-s", "scripts/tests", "-p", "test_ui_text_native.py", "-v"]),
        ("ui-text-coverage", ["python3", "-X", "dev", "-Werror", "-B",
                              "scripts/check-ui-text.py", "--root", "/work/fixture"]),
    ]


def native_input_control_commands(lane: str, *, schema_version: int) -> list[tuple[str, list[str]]]:
    """Run the frozen UI native-input source guard and controls before Cargo."""
    if type(schema_version) is not int or schema_version not in (2, 3):
        raise ENV.Incomplete("Unknown native-input verification schema.")
    if lane not in ("portable", "windows", "dependencies"):
        raise ENV.Incomplete("Unknown native-input verification row.")
    if schema_version == 2:
        return []
    return [("ui-native-input-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                          "discover", "-s", "scripts/tests", "-p", "test_native_input_source.py", "-v"])]


def validate_text_control_result(output: str) -> int:
    """Require actual nonempty unittest execution without ignored fault controls."""
    counts = re.findall(r"^Ran ([1-9][0-9]*) tests? in [^\n]+$", output, re.MULTILINE)
    if len(counts) != 1 or output.rstrip().splitlines()[-1:] != ["OK"]:
        raise ENV.Incomplete("UI text controls require nonempty unskipped passing execution.")
    return int(counts[0])


def validate_text_coverage_result(output: str) -> dict:
    """Retain explicit complete forward/reverse coverage, never an empty success."""
    if len(output.encode("utf-8")) > 65536:
        raise ENV.Incomplete("UI text coverage summary exceeds its byte budget.")
    try:
        result = json.loads(output)
    except (ValueError, RecursionError) as error:
        raise ENV.Incomplete("UI text coverage did not return a JSON summary.") from error
    if (type(result) is not dict or result.get("status") != "passed" or result.get("errors") != []
            or type(result.get("active_ids")) is not int or result["active_ids"] <= 0
            or type(result.get("used_ids")) is not int or result["used_ids"] != result["active_ids"]):
        raise ENV.Incomplete("UI text coverage requires nonempty complete forward/reverse success.")
    return result


def require_profile_lane(lane: str, profile: str) -> None:
    """Limit the newly reviewed provider profiles to dependency evidence."""
    if profile not in ("default", SLINT.EP1_PROFILE, *SLINT.RGT_PROFILES):
        raise ENV.Incomplete("Unknown dependency profile.")
    if profile in SLINT.RGT_PROFILES and lane != "dependencies":
        raise ENV.Incomplete("Roughtime profiles currently admit only the dependency lane.")


def commands(lane: str, *, schema_version: int = 2,
             patched_packages: tuple[str, ...] = (), profile: str = "default") -> list[tuple[str, list[str]]]:
    """Select explicit product commands; never replace execution by bootstrap controls."""
    if type(schema_version) is not int or schema_version not in (2, 3):
        raise ENV.Incomplete("Unknown product dependency schema.")
    if (type(patched_packages) is not tuple or patched_packages not in SLINT.PATCH_NAME_SETS
            or patched_packages and schema_version != 3):
        raise ENV.Incomplete("Unknown patched package verification scope.")
    require_profile_lane(lane, profile)
    if profile != "default" and schema_version != 3:
        raise ENV.Incomplete("An explicit dependency profile requires schema 3.")
    feature_argument: str | None = SLINT.profile_feature_argument(profile)
    cargo = ["cargo", "--config", 'source.crates-io.replace-with="reviewed"',
             "--config", 'source.reviewed.directory="/vendor"']

    def package_selection(package: str) -> str:
        """Keep NevroTick's estimator distinct from time's registry crate."""
        if package == "time-core" and SLINT.SLINT_CORE_NAME in patched_packages:
            return "time-core@0.0.0"
        return package

    base = ["--workspace", "--frozen", "--jobs", "12"]
    if schema_version == 3:
        targets = (HOST_TARGET, ENV.WINDOWS_TARGET) if lane == "dependencies" else (
            (HOST_TARGET,) if lane == "portable" else (ENV.WINDOWS_TARGET,) if lane == "windows" else ())
        if not targets:
            raise ENV.Incomplete("Unknown product verification row.")
        preflight = []
        for target in targets:
            preflight.extend([
                ("dependency-metadata-" + target, [*cargo, "metadata", "--frozen", "--offline",
                                                   "--format-version", "1", "--filter-platform", target]),
                ("dependency-tree-" + target, [*cargo, "tree", "--frozen", "--offline", "--target", target,
                                               "--workspace", "--edges", "normal,build", "--prefix", "indent",
                                               "--charset", "ascii", "--format", "{p}|{f}"]),
            ])
            for package in (*PACKAGES, *((SLINT.PATCH_NAME,) if SLINT.PATCH_NAME in patched_packages else ())):
                preflight.append(("dependency-tree-" + package + "-" + target,
                                  [*cargo, "tree", "--frozen", "--offline", "--target", target,
                                   "--package", package_selection(package), "--edges", "normal,build,dev", "--prefix", "indent",
                                   "--charset", "ascii", "--format", "{p}|{f}"]))
            if lane == "dependencies":
                preflight.append(("dependency-" + target, ["/tools/cargo-deny", "--frozen", "--target", target,
                                                           "--metadata-path", "/work/selected-metadata-" + target + ".json",
                                                           "check", "--deny", "warnings"]))
        if feature_argument is not None:
            for _, command in preflight:
                if command[0] == "cargo" and ("metadata" in command or "--workspace" in command
                        or "--package" in command and command[command.index("--package") + 1] == "nevrotick-engine"):
                    command.extend(["--features", feature_argument])
        if lane == "dependencies":
            return preflight
        patch_checks = []
        if SLINT.PATCH_NAME in patched_packages:
            manifest = "/work/fixture/" + SLINT.PATCH_PATH + "/Cargo.toml"
            compiler = [*cargo, "check", "--manifest-path", manifest, "--frozen", "--jobs", "6", "--all-targets"]
            if lane == "windows":
                compiler.extend(["--target", ENV.WINDOWS_TARGET])
            patch_checks.append(("patch-compiler", compiler))
            if lane == "portable":
                patch_checks.extend([
                    ("product-tests:spin_on", [*cargo, "nextest", "run", "--manifest-path", manifest,
                                               "--frozen", "--build-jobs", "6", "--lib", "--tests",
                                               "--config-file", "/work/fixture/.config/nextest.toml",
                                               "--user-config-file", "none", "--no-tests=fail"]),
                    ("doctests:spin_on", [*cargo, "test", "--manifest-path", manifest,
                                          "--frozen", "--jobs", "6", "--doc"]),
                ])
        ordinary = commands(lane)
        for _, command in ordinary:
            if "--package" in command:
                index = command.index("--package") + 1
                command[index] = package_selection(command[index])
        if lane == "portable":
            ordinary.extend([
                ("product-tests:" + SLINT.NATIVE_PACKAGE,
                 [*cargo, "nextest", "run", "--package", SLINT.NATIVE_PACKAGE,
                  "--frozen", "--build-jobs", "12", "--lib", "--tests",
                  "--user-config-file", "none", "--no-tests=fail"]),
                ("doctests:" + SLINT.NATIVE_PACKAGE,
                 [*cargo, "test", "--package", SLINT.NATIVE_PACKAGE,
                  "--frozen", "--jobs", "12", "--doc"]),
            ])
        if profile == SLINT.EP1_PROFILE:
            for _, command in ordinary:
                if command[0] == "cargo" and ("--workspace" in command
                        or "--package" in command and command[command.index("--package") + 1] == "nevrotick-engine"):
                    index = command.index("--") if "--" in command else len(command)
                    command[index:index] = ["--features", SLINT.EP1_FEATURE_ARGUMENT]
                    # Reduce concurrency after the EP1 compiler's actual task-
                    # resource failure; keep other rows and security flags intact.
                    job_flag = "--build-jobs" if "--build-jobs" in command else "--jobs"
                    command[command.index(job_flag) + 1] = str(TEST_BUILD_JOBS)
        return preflight + patch_checks + ordinary
    if lane == "portable":
        return [
            ("format", ["cargo", "fmt", "--all", "--", "--check"]),
            ("compiler", [*cargo, "check", *base, "--all-targets"]),
            ("clippy", [*cargo, "clippy", *base, "--all-targets", "--", "-D", "warnings"]),
            ("host-engine-build", [*cargo, "build", "--package", "nevrotick-engine", "--frozen",
                                   "--jobs", "12", "--bin", "nevrotick-observe"]),
            ("rustdoc", [*cargo, "doc", *base, "--lib", "--no-deps"]),
        ] + [
            ("product-tests:" + package, [*cargo, "nextest", "run", "--package", package,
                                         "--frozen", "--build-jobs",
                                         str(TEST_BUILD_JOBS) if package in BOUNDED_TEST_BUILD_PACKAGES else "12",
                                         "--lib", "--tests",
                                         "--user-config-file", "none", "--no-tests=fail"])
            for package in LEGACY_PACKAGES
        ] + [("doctests:" + package, [*cargo, "test", "--package", package,
                                      "--frozen", "--jobs", "12", "--doc"])
             for package in LEGACY_PACKAGES] + [
            ("optimized-tests:" + package, [*cargo, "nextest", "run", "--package", package,
                                           "--cargo-profile", "release", "--frozen", "--build-jobs",
                                           str(TEST_BUILD_JOBS) if package in BOUNDED_TEST_BUILD_PACKAGES else "12",
                                           "--lib", "--tests",
                                           "--user-config-file", "none", "--no-tests=fail"])
            for package in OPTIMIZED_DISCRIMINATORS]
    if lane == "windows":
        target = [*base, "--target", ENV.WINDOWS_TARGET]
        return [("windows-compiler", [*cargo, "check", *target, "--all-targets"]),
                ("windows-clippy", [*cargo, "clippy", *target, "--all-targets", "--", "-D", "warnings"]),
                 ("windows-link", [*cargo, "--config", f'target.{ENV.WINDOWS_TARGET}.linker="/work/linker"',
                                   "build", "--release", "--package", PACKAGE, "--frozen", "--jobs", "12",
                                   "--target", ENV.WINDOWS_TARGET, "--bin", PACKAGE]),
                ("windows-engine-link", [*cargo, "--config", f'target.{ENV.WINDOWS_TARGET}.linker="/work/linker"',
                                         "build", "--release", "--package", "nevrotick-engine", "--frozen", "--jobs", "12",
                                         "--target", ENV.WINDOWS_TARGET, "--bin", "nevrotick-observe"])]
    if lane == "dependencies":
        rows = []
        for target in (HOST_TARGET, ENV.WINDOWS_TARGET):
            rows.extend([
                ("dependency-metadata-" + target, [*cargo, "metadata", "--frozen", "--offline",
                                                   "--format-version", "1", "--filter-platform", target]),
                ("dependency-" + target, ["/tools/cargo-deny", "--frozen", "--target", target,
                                          "check", "--deny", "warnings"]),
            ])
        return rows
    raise ENV.Incomplete("Unknown product verification row.")


def windows_pe_reader(source: Path):
    """Load the PE and PDB reader from the admitted source snapshot."""
    spec = importlib.util.spec_from_file_location("product_pe", source / "scripts/windows_pe.py")
    reader = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reader)
    return reader


def windows_versioninfo_reader(source: Path) -> ModuleType:
    """Load the resource payload decoder from the selected source."""
    spec: importlib.machinery.ModuleSpec | None = importlib.util.spec_from_file_location(
        "product_versioninfo", source / "scripts/windows_versioninfo.py")
    if spec is None or spec.loader is None:
        raise ENV.Incomplete("Windows resource decoder is unavailable.")
    reader: ModuleType = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reader)
    return reader


def export_windows_artifacts(work: Path, output: Path, *, source: Path,
                             ui_sha256: str | None = None, observe_sha256: str | None = None,
                             final_output: Path | None = None) -> dict:
    """Export both release executables and their unique PDBs to fresh output.

    Admit both complete pairs before copying their exact checked snapshots.
    Report final_output paths when the caller supplies an owned staging slot.
    """
    build = work / "target" / ENV.WINDOWS_TARGET / "release"
    pe = windows_pe_reader(source)
    artifacts, payloads = {}, {}
    for role, name, subsystem in (("ui", PACKAGE, "gui"), ("observer", "nevrotick-observe", "console")):
        binary = build / f"{name}.exe"
        names = {f"{name}.pdb", f"{name.replace('-', '_')}.pdb"}
        symbol_paths = [build / candidate for candidate in sorted(names) if (build / candidate).is_file()]
        if len(symbol_paths) != 1:
            raise ENV.Incomplete("Release Windows artifact needs exactly one matching PDB.")
        pdb = symbol_paths[0]
        try:
            image, symbols, identity = pe.read_export_pair(binary, pdb, subsystem=subsystem)
        except ValueError as error:
            raise ENV.Incomplete(str(error)) from error
        if role == "ui" and ui_sha256 is not None and image[1] != ui_sha256:
            raise ENV.Incomplete("Windows UI bytes changed after version resource inspection.")
        if role == "observer" and observe_sha256 is not None and image[1] != observe_sha256:
            raise ENV.Incomplete("Windows Observe bytes changed after version resource inspection.")
        payloads.update({binary: image, pdb: symbols})
        artifacts[role] = (binary, pdb, identity)
    if any((output / path.name).exists() or (output / path.name).is_symlink() for path in payloads):
        raise ENV.Incomplete("Windows artifact export refuses existing destinations.")
    for path, (data, expected) in payloads.items():
        destination = output / path.name
        with destination.open("xb") as target:
            target.write(data)
        if sha256(destination) != expected or sha256(path) != expected:
            raise ENV.Incomplete("Windows artifact bytes changed during export.")
    result = {}
    final = output if final_output is None else final_output
    for role, (binary, pdb, symbols) in artifacts.items():
        entry = {"path": str(final / binary.name), "sha256": payloads[binary][1],
                 "profile": "release", "target": ENV.WINDOWS_TARGET,
                 "component_role": role, "source_state": "fixture" if role == "ui" else "experiment",
                 "symbols": {"path": str(final / pdb.name), **symbols}}
        if role == "ui":
            result.update(entry)
        else:
            # Preserve the original receipt key for the Observe executable.
            result["engine"] = entry
    return result


class WindowsOutputPublicationUncertain(ENV.Incomplete):
    """A complete export became visible but its publication is unconfirmed."""


def _publish_windows_export(parent: int, staging: str, name: str) -> None:
    """Rename within the held Linux parent without replacing any destination."""
    try:
        rename = ctypes.CDLL(None, use_errno=True).renameat2
    except AttributeError as error:
        raise OSError(errno.ENOSYS, "Atomic Windows export publication is unavailable.") from error
    rename.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
    rename.restype = ctypes.c_int
    rename_noreplace = 1
    if rename(parent, os.fsencode(staging), parent, os.fsencode(name), rename_noreplace) != 0:
        code = ctypes.get_errno()
        raise OSError(code, os.strerror(code), name)


def _flush_windows_export(staging_descriptor: int) -> None:
    """Flush every staged regular file and all containing directories before rename."""
    flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
    def refuse(error: OSError) -> None:
        raise error

    for _, directories, files, descriptor in os.fwalk(".", dir_fd=staging_descriptor, topdown=False, onerror=refuse):
        for name in directories:
            if not stat.S_ISDIR(os.stat(name, dir_fd=descriptor, follow_symlinks=False).st_mode):
                raise ENV.Incomplete("Unexpected linked directory in Windows export.")
        for name in files:
            file_descriptor = os.open(name, flags, dir_fd=descriptor)
            try:
                if not stat.S_ISREG(os.fstat(file_descriptor).st_mode):
                    raise ENV.Incomplete("Unexpected nonregular file in Windows export.")
                os.fsync(file_descriptor)
            finally:
                os.close(file_descriptor)
        os.fsync(descriptor)


def export_windows_build(work: Path, output: Path, *, source: Path, report: dict,
                         base_artwork: dict[str, bytes] | None,
                         authoring: dict[str, bytes] | None) -> None:
    """Publish a complete checked handoff to a new external Linux output slot.

    Precommit failure retires private staging. After rename, preserve the
    complete tree on a parent flush or path failure and report uncertainty.
    The caller supplies the resolved parent and the original output leaf.
    """
    inspection = report.get("msix_assembly_inputs")
    if inspection is None and (base_artwork is not None or authoring is not None):
        raise ENV.Incomplete("Windows export lacks its checked MSIX authoring generation.")
    parent = os.open(output.parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    staging_descriptor = None
    try:
        def require_parent() -> None:
            info = os.fstat(parent)
            shared_writes = info.st_mode & (stat.S_IWGRP | stat.S_IWOTH)
            if info.st_uid not in {0, os.geteuid()} or (shared_writes and not info.st_mode & stat.S_ISVTX):
                raise ENV.Incomplete("Windows export requires a trusted parent with sticky shared writes.")
            if not os.path.samestat(output.parent.stat(follow_symlinks=False), info):
                raise ENV.Incomplete("Windows export parent changed during publication.")

        require_parent()
        # The held directory also anchors staging cleanup if the named parent moves.
        parent_anchor = Path(f"/proc/self/fd/{parent}")
        if (parent_anchor / output.name).exists() or (parent_anchor / output.name).is_symlink():
            raise FileExistsError(errno.EEXIST, os.strerror(errno.EEXIST), output.name)
        anchored = Path(tempfile.mkdtemp(prefix=".windows-export-", dir=parent_anchor))
        staged_identity: os.stat_result | None = None
        try:
            staged_identity = anchored.stat(follow_symlinks=False)
            staging_descriptor = os.open(anchored.name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
                                       | os.O_CLOEXEC, dir_fd=parent)
            if not os.path.samestat(staged_identity, os.fstat(staging_descriptor)):
                raise ENV.Incomplete("Windows export staging changed before writing.")
            stage = Path(f"/proc/self/fd/{staging_descriptor}")
            require_parent()
            completed = {**report, "status": "passed"}
            completed["artifact"] = export_windows_artifacts(
                work, stage, source=source, final_output=output,
                ui_sha256=report.get("ui_versioninfo", {}).get("image_sha256"),
                observe_sha256=report.get("observe_versioninfo", {}).get("image_sha256"))
            if "ui_preentry" in report:
                entry: dict[str, Any] = report["ui_preentry"]
                image_pin: str = completed["artifact"]["sha256"]
                source_map: Path = work / Path(UI_ENTRY_MAP).relative_to("/work")
                map_bytes: bytes = read_bound_ui_entry_map(source_map, entry, image_pin)
                exported_map: Path = stage / UI_ENTRY_MAP_LEAF
                with exported_map.open("xb") as stream:
                    stream.write(map_bytes)
                read_bound_ui_entry_map(exported_map, entry, image_pin, staging_descriptor=staging_descriptor)
                completed["ui_preentry"] = {**entry, "map_artifact": UI_ENTRY_MAP_LEAF}
            if inspection is not None:
                inspection = dict(inspection)
                if base_artwork is not None:
                    inspection["base_artwork"] = export_msix_artwork(
                        base_artwork, stage, staging_descriptor=staging_descriptor)
                if authoring is not None:
                    inspection["authoring_export"] = export_msix_authoring(
                        authoring, stage, inspection=inspection, staging_descriptor=staging_descriptor,
                        properties_source=source if base_artwork is not None else None)
                completed["msix_assembly_inputs"] = inspection
            receipt = stage / "verification.json"
            receipt_data = (json.dumps(completed, indent=2) + "\n").encode("utf-8")
            with receipt.open("xb") as stream:
                stream.write(receipt_data)
            require_parent()
            with receipt.open("rb") as stream:
                if stream.read(len(receipt_data) + 1) != receipt_data:
                    raise ENV.Incomplete("Windows export receipt changed during publication.")
            _flush_windows_export(staging_descriptor)
            require_parent()
            if not os.path.samestat(staged_identity, anchored.stat(follow_symlinks=False)):
                raise ENV.Incomplete("Windows export staging changed before publication.")
            _publish_windows_export(parent, anchored.name, output.name)
        except BaseException as error:
            try:
                if staged_identity is None:
                    error.add_note("Windows export staging identity could not be established. Unknown object preserved.")
                elif os.path.samestat(staged_identity, anchored.stat(follow_symlinks=False)):
                    shutil.rmtree(anchored)
                else:
                    error.add_note("Windows export staging changed before cleanup. Replacement preserved.")
            except OSError as cleanup:
                error.add_note(f"Windows export staging cleanup failed ({type(cleanup).__name__}).")
            raise
        try:
            os.fsync(parent)
            require_parent()
            if not os.path.samestat(staged_identity, os.stat(output.name, dir_fd=parent, follow_symlinks=False)):
                raise ENV.Incomplete("Windows export final slot changed during publication.")
        except (OSError, ENV.Incomplete) as error:
            message = "Windows export rename completed. Publication durability or path is unconfirmed."
            report.update(status="publication-uncertain", error=message)
            raise WindowsOutputPublicationUncertain(message) from error
        report.update(completed)
    finally:
        try:
            if staging_descriptor is not None:
                os.close(staging_descriptor)
        finally:
            os.close(parent)


def export_host_engine_artifact(work: Path, output: Path) -> dict:
    """Retain only the current host build, checking bytes and executable ownership.

    Reject absent/linked/nonexecutable source and existing destination before
    copying. Export to an exclusively created regular file, restrict execution
    to its owner, then verify unchanged source and exact copied bytes. This is
    artifact custody; it does not execute or qualify transport behavior.
    """
    source = work / "target/debug/nevrotick-observe"
    if (source.is_symlink() or not source.is_file() or source.resolve() != source
            or not source.stat().st_mode & 0o100):
        raise ENV.Incomplete("Current host engine build must be a regular nonlinked owner-executable file.")
    destination = output / "nevrotick-observe"
    if destination.exists() or destination.is_symlink():
        raise ENV.Incomplete("Host engine export refuses an existing destination.")
    expected = sha256(source)
    with source.open("rb") as origin, destination.open("xb") as target:
        shutil.copyfileobj(origin, target)
    destination.chmod(0o700)
    if sha256(source) != expected or sha256(destination) != expected:
        raise ENV.Incomplete("Host engine source or copied bytes changed during export.")
    return {"path": str(destination), "sha256": expected, "profile": "dev",
            "target": HOST_TARGET, "binary_name": "nevrotick-observe"}


def require_slint_capture(args, policy: dict) -> bool:
    """Admit optional offscreen capture only with explicit portable SDK/output."""
    requested = getattr(args, "capture_slint", False)
    if type(requested) is not bool:
        raise ENV.Incomplete("Slint capture requires an explicit boolean opt-in.")
    if requested and (args.lane != "portable" or policy.get("schema_version") != 3
                      or getattr(args, "linux_sdk_dir", None) is None
                      or getattr(args, "output_dir", None) is None):
        raise ENV.Incomplete("Slint capture requires portable schema 3, the admitted SDK and new output.")
    return requested


def slint_capture_sizes() -> dict[str, tuple[int, int]]:
    """Return the exact 50-frame matrix, including grouped preferences and modals."""
    names = {f"{mode}-{page}-{width}.ppm": width
             for width in (640, 1080)
             for mode, pages in (("simple", ("status", "settings", "about")),
                                 ("power", ("status", "sources", "statistics", "diagnostics", "settings", "about")))
             for page in pages}
    names |= {"source-invalid-1080.ppm": 1080, "interval-invalid-1080.ppm": 1080,
                    "interval-invalid-640.ppm": 640, "source-add-ntp-1080.ppm": 1080,
                    "source-review-1080.ppm": 1080, "source-empty-640.ppm": 640,
                    "source-empty-1080.ppm": 1080}
    sizes = {name: (width, 780) for name, width in names.items()}
    sizes.update({"recovery-640.ppm": (640, 780), "recovery-1080.ppm": (1080, 780),
                  "recovery-invalid-640.ppm": (640, 780),
                  "recovery-invalid-640x600.ppm": (640, 600),
                  "interval-invalid-640x600.ppm": (640, 600),
                  "notifications-simple-640x600.ppm": (640, 600),
                  "personal-simple-640x600.ppm": (640, 600),
                  "recovery-leave-640x600.ppm": (640, 600),
                  "quit-warning-top-640x600.ppm": (640, 600),
                  "quit-warning-bottom-640x600.ppm": (640, 600),
                  "quit-warning-long-top-640x600.ppm": (640, 600),
                  "quit-warning-long-bottom-640x600.ppm": (640, 600),
                  "service-simple-640.ppm": (640, 780),
                  "service-power-640x600.ppm": (640, 600),
                  "diagnostic-report-640.ppm": (640, 780),
                  "diagnostic-report-1080.ppm": (1080, 780),
                  "diagnostic-collection-640x600.ppm": (640, 600),
                  "diagnostic-correction-640x600.ppm": (640, 600),
                  "policy-ntp-review-640x600.ppm": (640, 600),
                  "policy-ntp-review-1080x780.ppm": (1080, 780),
                  "policy-observe-review-640x600.ppm": (640, 600),
                  "policy-observe-review-1080x780.ppm": (1080, 780),
                  "sources-query-640x600.ppm": (640, 600),
                  "sources-no-match-640x600.ppm": (640, 600),
                  "source-details-640x600.ppm": (640, 600)})
    return sizes


def slint_capture_build_command() -> list[str]:
    """Bound debug information only for this developer capture invocation.

    The fixed global config keeps ordinary dev compilation while suppressing UI
    package debug data in its library and capture example. Final-only stripping
    also keeps the linked capture small. Manifest profiles, assertions, overflow
    checks and sandbox limits stay unchanged; other build rows get no override.
    """
    return ["cargo", "--config", "profile.dev.package.nevrotick-ui.debug=0", "rustc",
            "--frozen", "--offline", "--jobs", "6",
            "--package", PACKAGE, "--example", "capture", "--", "-C", "strip=debuginfo"]


def export_slint_captures(work: Path, output: Path) -> dict:
    """Validate the entire bounded frame set before exporting any capture."""
    source = work / "slint-captures"
    expected = slint_capture_sizes()
    if (source.is_symlink() or not source.is_dir()
            or {path.name for path in source.iterdir()} != set(expected)):
        raise ENV.Incomplete("Slint capture needs its exact selected frame inventory.")
    records = {}
    for name, (width, height) in expected.items():
        path = source / name
        header = f"P6\n{width} {height}\n255\n".encode("ascii")
        if path.is_symlink() or not path.is_file() or path.stat().st_size != len(header) + width * height * 3:
            raise ENV.Incomplete("Slint capture is linked, truncated or has an unselected size.")
        with path.open("rb") as stream:
            if stream.read(len(header)) != header:
                raise ENV.Incomplete("Slint capture has an unselected PPM header.")
        records[name] = {"sha256": sha256(path), "width": width, "height": height}
    destination = output / "slint-captures"
    destination.mkdir(mode=0o700)
    for name, record in records.items():
        shutil.copyfile(source / name, destination / name)
        if sha256(destination / name) != record["sha256"]:
            raise ENV.Incomplete("Slint capture changed during export.")
    return {"scope": "Public software platform on Linux; no native Windows observation", "files": records}


def planned_test_cases(lane: str, profile: str) -> set[CaseIdentity]:
    """Select claim-critical cases for the current portable package/profile row."""
    kind, _, package = lane.partition(":")
    required: set[CaseIdentity] = set()
    if kind not in {"product-tests", "optimized-tests"}:
        return required
    if package == "nevrotick-engine":
        required.update((package, case) for case in {ENGINE_PROCESSING_GUARD, *ENGINE_RETAINED_REPLY_CASES})
        if profile == SLINT.EP1_PROFILE:
            required.update((package, case) for case in ENGINE_KE_TEST_CASES)
    if package == SLINT.PATCH_NAME:
        required.update(PATCH_TEST_CASES)
    if kind == "optimized-tests" and package in OPTIMIZED_DISCRIMINATORS:
        required.add(OPTIMIZED_DISCRIMINATORS[package])
    return required


def nextest_discovery_command(command: list[str]) -> list[str]:
    """Use the run's exact Cargo build and filter selection for discovery."""
    index: int = command.index("nextest") + 1
    if command[index] != "run" or command.count("--no-tests=fail") != 1:
        raise ENV.Incomplete("Nextest discovery needs an admitted product test command.")
    listing: list[str] = list(command)
    listing[index] = "list"
    listing.remove("--no-tests=fail")
    return [*listing, "--message-format", "json"]


def test_suite_belongs_to(suite: str, package: str) -> bool:
    return suite == package or (suite.startswith(package + "::") and bool(suite[len(package) + 2:]))


def validate_test_discovery(lane: str, output: str, profile: str) -> set[CaseIdentity]:
    """Read the pinned nextest JSON listing, independently of JUnit execution."""
    _, _, package = lane.partition(":")

    def unique_fields(pairs: list[tuple[str, object]]) -> dict[str, object]:
        fields: dict[str, object] = dict(pairs)
        if len(fields) != len(pairs):
            raise ValueError("duplicate nextest discovery field")
        return fields

    # The pinned compact format emits one JSON line. Cargo and archive status
    # messages share this runner's bounded stream but are not discovery records.
    records: list[str] = [line for line in output.splitlines() if line.startswith("{")]
    if len(records) != 1:
        raise ENV.Incomplete("Nextest discovery needs one compact JSON record.")
    try:
        record: object = json.loads(records[0], object_pairs_hook=unique_fields)
    except ValueError as error:
        raise ENV.Incomplete("Invalid nextest discovery record.") from error
    if type(record) is not dict:
        raise ENV.Incomplete("Nextest discovery has no test binary inventory.")
    inventory: object = record.get("rust-suites")
    if type(inventory) is not dict:
        raise ENV.Incomplete("Nextest discovery has no test binary inventory.")
    # Every JSON object passes through unique_fields, with string keys.
    binaries: dict[str, object] = inventory
    discovered: set[CaseIdentity] = set()
    total: int = 0
    for suite, binary in binaries.items():
        if (not test_suite_belongs_to(suite, package) or type(binary) is not dict
                or binary.get("binary-id") != suite or binary.get("package-name") != package
                or type(binary.get("testcases")) is not dict):
            raise ENV.Incomplete("Nextest discovery contains an invalid or foreign test binary.")
        cases: dict[str, object] = binary["testcases"]
        binary_status: object = binary.get("status")
        if binary_status == "skipped" and not cases:
            continue
        if binary_status != "listed":
            raise ENV.Incomplete("Nextest discovery contains an invalid test binary status.")
        for name, case in cases.items():
            total += 1
            if not name.strip() or type(case) is not dict:
                raise ENV.Incomplete("Nextest discovery contains an invalid testcase identity or selection.")
            ignored: object = case.get("ignored")
            match: object = case.get("filter-match")
            if type(ignored) is not bool or type(match) is not dict:
                raise ENV.Incomplete("Nextest discovery contains an invalid testcase identity or selection.")
            status: object = match.get("status")
            if status == "matches" and not ignored:
                discovered.add((suite, name))
            elif status != "mismatch":
                raise ENV.Incomplete("Nextest discovery contains an inadmissible testcase selection.")
    declared_total: object = record.get("test-count")
    if type(declared_total) is not int or total != declared_total:
        raise ENV.Incomplete("Nextest discovery count differs from its binary inventory.")
    if not planned_test_cases(lane, profile) <= discovered:
        raise ENV.Incomplete("Nextest discovery lacks mandatory qualified testcases.")
    return discovered


def validate_test_result(lane: str, output: str, work: Path, *, profile: str = "default") -> dict | None:
    """Credit successful JUnit cases only under their package and binary identity."""
    if profile not in ("default", SLINT.EP1_PROFILE, *SLINT.RGT_PROFILES):
        raise ENV.Incomplete("Unknown test-accounting profile.")
    kind, _, package = lane.partition(":")
    if kind not in {"product-tests", "doctests", "optimized-tests"}:
        return None
    if profile in SLINT.RGT_PROFILES:
        raise ENV.Incomplete("Roughtime dependency profiles cannot credit product tests.")
    if package not in {*PACKAGES, SLINT.PATCH_NAME}:
        raise ENV.Incomplete("Test accounting identifies an unreviewed package.")
    if kind == "optimized-tests" and package not in OPTIMIZED_DISCRIMINATORS:
        raise ENV.Incomplete("Optimized test accounting identifies an unreviewed package/profile.")
    if kind in {"product-tests", "optimized-tests"}:
        report = work / "target/nextest/default/junit.xml"
        if not report.is_file():
            raise ENV.Incomplete("Product test execution lacks its JUnit accounting.")
        try:
            root = ET.parse(report).getroot()
        except ET.ParseError as error:
            raise ENV.Incomplete("Product test execution emitted invalid JUnit accounting.") from error
        suites: list[ET.Element] = list(root.iter("testsuite"))
        cases: list[tuple[str, ET.Element]] = [(suite.get("name", ""), case)
                                              for suite in suites for case in suite.findall("testcase")]
        if (not cases or len(cases) != len(list(root.iter("testcase")))
                or any(not test_suite_belongs_to(suite.get("name", ""), package) for suite in suites)
                or any(any(child.tag in {"skipped", "failure", "error"} for child in case) for _, case in cases)):
            raise ENV.Incomplete("Required product tests are empty, skipped or failed.")
        executed: list[CaseIdentity] = []
        for suite, case in cases:
            name: str = case.get("name", "")
            if not name.strip() or case.get("classname", suite) != suite:
                raise ENV.Incomplete("Missing or inconsistent JUnit testcase identity.")
            executed.append((suite, name))
        unique: set[CaseIdentity] = set(executed)
        if len(unique) != len(executed):
            raise ENV.Incomplete("Duplicate qualified JUnit testcase identity.")
        planned: set[CaseIdentity] = planned_test_cases(lane, profile)
        if not planned <= unique:
            raise ENV.Incomplete("JUnit execution lacks mandatory qualified testcases.")
        return {"package": package, "executed": len(executed), "test_names": [name for _, name in executed],
                "planned_cases": sorted(planned), "executed_cases": executed}
    if kind == "doctests":
        counts = re.findall(r"test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;", output)
        if not counts or sum(int(passed) for passed, _ in counts) == 0 or any(int(ignored) for _, ignored in counts):
            raise ENV.Incomplete("Required doctests did not execute a nonempty passing selection.")
        return {"package": package, "executed": sum(int(passed) for passed, _ in counts)}
    return None


def clear_test_report(lane: str, work: Path) -> None:
    """Remove a prior package's report so it cannot stand in for a missing run."""
    if lane.startswith(("product-tests:", "optimized-tests:")):
        report = work / "target/nextest/default/junit.xml"
        if report.is_symlink():
            raise ENV.Incomplete("A linked product test report is not admitted.")
        report.unlink(missing_ok=True)


def summarize_test_counts(records: list[dict], *, patched_packages: tuple[str, ...] = (),
                          schema_version: int = 2) -> dict:
    """Require ordinary, doctest and selected optimized evidence before aggregating."""
    if type(patched_packages) is not tuple or patched_packages not in SLINT.PATCH_NAME_SETS:
        raise ENV.Incomplete("Unknown patched test-accounting owner.")
    if type(schema_version) is not int or schema_version not in (2, 3):
        raise ENV.Incomplete("Unknown product test-accounting schema.")
    # AccessKit is selected only by the Windows GUI graph, never a standalone test owner.
    standalone = (SLINT.PATCH_NAME,) if SLINT.PATCH_NAME in patched_packages else ()
    members = PACKAGES if schema_version == 3 else LEGACY_PACKAGES
    counts = {package: {} for package in (*members, *standalone)}
    for record in records:
        kind, _, package = record["lane"].partition(":")
        if kind not in {"product-tests", "doctests", "optimized-tests"}:
            continue
        result = record.get("tests", {})
        if (package not in counts or kind in counts[package] or result.get("package") != package
                or type(result.get("executed")) is not int or result["executed"] <= 0
                or kind == "optimized-tests" and package not in OPTIMIZED_DISCRIMINATORS):
            raise ENV.Incomplete("Per-package product test accounting is incomplete or duplicated.")
        counts[package][kind] = result["executed"]
    for package, result in counts.items():
        required = {"product-tests", "doctests"} | ({"optimized-tests"} if package in OPTIMIZED_DISCRIMINATORS else set())
        if set(result) != required:
            raise ENV.Incomplete("Every package requires ordinary tests/doctests and each selected owner requires optimized tests.")
    return {"by_package": counts,
            "tests_executed": sum(result["product-tests"] for result in counts.values()),
            "doctests_executed": sum(result["doctests"] for result in counts.values()),
            "optimized_tests_executed": sum(counts[package]["optimized-tests"] for package in OPTIMIZED_DISCRIMINATORS)}


def observe_resource_path(digest: str) -> str:
    """Make the selected resource bytes part of the leaf invocation identity."""
    if re.fullmatch(r"[a-f0-9]{64}", digest) is None:
        raise ENV.Incomplete("Invalid Observe resource digest.")
    return f"/work/metadata/observe-{digest}.res"


def link_observe_resource(build: list[str], digest: str) -> list[str]:
    """Pass the resource to the exact executable's final rustc invocation."""
    command = list(build)
    command[command.index("build")] = "rustc"
    return [*command, "--", "-C", "link-arg=" + observe_resource_path(digest)]


def ui_entry_object_path(digest: str) -> str:
    """Bind the checked baseline object to the final executable's Cargo input."""
    if re.fullmatch(r"[a-f0-9]{64}", digest) is None:
        raise ENV.Incomplete("Invalid UI baseline entry object digest.")
    return f"/work/metadata/nevrotick-ui-entry-{digest}.obj"


def link_ui_entry(build: list[str], digest: str) -> list[str]:
    """Select only the UI executable's baseline entry and map."""
    command: list[str] = list(build)
    command[command.index("build")] = "rustc"
    return [*command, "--", "-C", "link-arg=" + ui_entry_object_path(digest),
            "-C", "link-arg=/entry:nevrotick_ui_entry", "-C", "link-arg=/map:" + UI_ENTRY_MAP]


def command_resources(lane: str, label: str, command: list[str], *, profile: str = "default") -> dict:
    """Select one finite build budget from the exact configured command owner.

    The two Windows release artifact rows and the portable EP1 archive build
    have fixed compilation ceilings. Windows UI alone allows 8 GiB AS and 600 seconds wall time.
    There is no caller-provided number, environment override or CLI budget.
    """
    if (type(lane) is not str or lane not in {"portable", "windows", "dependencies"}
            or type(profile) is not str
            or type(label) is not str or not label
            or type(command) is not list or not command
            or any(type(argument) is not str for argument in command)):
        raise ENV.Incomplete("Unadmitted product command resource context.")
    require_profile_lane(lane, profile)
    portable_ep1 = lane == "portable" and profile == SLINT.EP1_PROFILE
    if label == ENGINE_EP1_RELEASE_BUILD_ROW or tuple(command) == ENGINE_EP1_RELEASE_BUILD:
        if not portable_ep1 or label != ENGINE_EP1_RELEASE_BUILD_ROW or tuple(command) != ENGINE_EP1_RELEASE_BUILD:
            raise ENV.Incomplete("Engine EP1 release build requires its exact lane, profile, row and command.")
        return {"name": "product-gnu-engine-ep1-release-build", "cpu_seconds": 240, "wall_seconds": 300}

    release_labels = {"windows-link", "windows-engine-link"}
    if label in release_labels or "--release" in command:
        if lane != "windows" or label not in release_labels:
            raise ENV.Incomplete("Windows release resources require their exact lane and row.")
        expected = dict(commands("windows", schema_version=3, profile=profile))[label]
        if label == "windows-engine-link":
            resource = re.fullmatch(r"link-arg=/work/metadata/observe-([a-f0-9]{64})\.res", command[-1])
            if resource is not None:
                expected = link_observe_resource(expected, resource[1])
        elif len(command) >= 6:
            entry: re.Match[str] | None = re.fullmatch(
                r"link-arg=/work/metadata/nevrotick-ui-entry-([a-f0-9]{64})\.obj", command[-5])
            if entry is not None:
                expected = link_ui_entry(expected, entry[1])
        if command != expected:
            raise ENV.Incomplete("Windows release resource command differs from its fixed owner.")
        if label == "windows-link":
            return {"name": "product-windows-ui-release", "cpu_seconds": 300, "wall_seconds": 600,
                    "address_space_bytes": 8589934592}
        return {"name": "product-windows-release", "cpu_seconds": 300, "wall_seconds": 300}
    return {"name": "bootstrap", "cpu_seconds": 90, "wall_seconds": 300}


def sandbox(toolchain: Path, work: Path, source: Path, vendor: Path,
            command: list[str], extension: Path | None, libraries: Path | None = None,
            extension_name: str = "cargo-nextest", linux_sdk: Path | None = None, *,
            lane: str | None = None, label: str | None = None, profile: str = "default") -> list[str]:
    """Mount only reviewed inputs and owned output; keep the source snapshot read-only."""
    resources = None
    if lane is not None or label is not None:
        resources = command_resources(lane, label, command, profile=profile)
    elif profile != "default":
        raise ENV.Incomplete("A dependency profile requires an explicit product resource context.")
    invocation = ENV.sandbox_command(toolchain, work, command)
    if resources is not None and resources["cpu_seconds"] != 90:
        # The bootstrap owner remains unchanged. Admit its complete current
        # wrapper before adapting CPU and, for the exact UI release row, AS.
        try:
            separator = invocation.index("--")
        except ValueError as error:
            raise ENV.Incomplete("Release compilation lacks the admitted bootstrap wrapper.") from error
        expected = ["/usr/bin/prlimit", "--as=4294967296", "--cpu=90", "--nproc=128",
                    "--nofile=256", "--fsize=268435456", "--", *command]
        if invocation[separator + 1:] != expected:
            raise ENV.Incomplete("Release compilation has a changed bootstrap resource wrapper.")
        invocation[separator + 3] = f"--cpu={resources['cpu_seconds']}"
        if resources["name"] == "product-windows-ui-release":
            invocation[separator + 2] = "--as=8589934592"
    index = invocation.index("--")
    extra = ["--ro-bind", str(source), "/work/fixture", "--ro-bind", str(vendor), "/vendor",
             "--dir", "/tools", "--setenv", "PATH", "/tools:/toolchain/bin:/usr/bin:/bin"]
    if extension:
        extra += ["--ro-bind", str(extension), "/tools/" + extension_name]
    if libraries:
        extra += ["--ro-bind", str(libraries), "/xwin"]
    if linux_sdk:
        extra += SDK.sandbox_arguments(linux_sdk)
    return invocation[:index] + extra + invocation[index:]


def boundary_control(run) -> None:
    """Challenge filesystem, inherited environment and host-network isolation before Cargo."""
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        listener.listen(1)
        port = listener.getsockname()[1]
        with socket.create_connection(("127.0.0.1", port), timeout=2):
            accepted, _ = listener.accept()
            accepted.close()
        probe = (
            "import os,pathlib,socket; "
            "assert 'HOME' not in os.environ; "
            "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ; "
            "assert not pathlib.Path('/run/docker.sock').exists(); "
            f"assert not pathlib.Path({str(Path.home())!r}).exists(); "
            "s=socket.socket(); s.settimeout(1)\n"
            "pathlib.Path('/work/output-canary').write_text('owned writable output')\n"
            "pathlib.Path('/work/output-canary').unlink()\n"
            "try: pathlib.Path('/work/fixture/source-write-canary').write_text('must fail')\n"
            "except OSError: pass\n"
            "else: raise SystemExit('source snapshot is writable')\n"
            f"try: s.connect(('127.0.0.1',{port}))\n"
            "except OSError: print('PRODUCT_BOUNDARY_OK')\n"
            "else: raise SystemExit('host network visible')\n"
            "finally: s.close()\n"
        )
        if "PRODUCT_BOUNDARY_OK" not in run("boundary-control", ["python3", "-c", probe]):
            raise ENV.Incomplete("The isolation observer did not complete.")


def library_inventory(directory: Path) -> dict[str, str]:
    """Hash all selected Microsoft libraries, including filenames and directory scope."""
    result = {}
    for suffix in ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64"):
        for name, digest in inventory(directory / suffix).items():
            result[suffix + "/" + name] = digest
    if not result:
        raise ENV.Incomplete("Microsoft library inventory is empty.")
    return result


def prepare_windows_link(root: Path, toolchain: Path, config_path: Path, work: Path,
                         inventory_path: Path | None) -> tuple[Path, dict[str, str], tuple[str, Path, str]]:
    """Admit the configured Microsoft input identity and pinned bundled linker."""
    policy = json.loads((root / "policy/tool-manifest.json").read_text())
    config = json.loads(config_path.read_text())
    expected = policy["cross_toolchain"]["microsoft_inputs"]
    for key in ("architecture", "variant", "sdk_version", "crt_version", "channel_manifest_sha256",
                "package_manifest_sha256", "package_manifest_declared_sha256"):
        if config["microsoft_inputs"][key] != expected[key]:
            raise ENV.Incomplete("Microsoft input identity differs from the reviewed cross lane.")
    directory = Path(config["root"]).resolve(strict=True)
    suffixes = ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")
    if [str(directory / suffix) for suffix in suffixes] != config["library_paths"]:
        raise ENV.Incomplete("Unexpected Microsoft library search paths.")
    for suffix, name in zip(suffixes, ("msvcrt.lib", "kernel32.lib", "ucrt.lib")):
        sha256(directory / suffix / name)
    if inventory_path is None:
        raise ENV.Incomplete("Windows linking requires the admitted complete library inventory.")
    files = library_inventory(directory)
    acquisition = json.loads(inventory_path.read_text())
    if acquisition.get("acquisition_receipt_sha256") != sha256(config_path):
        raise ENV.Incomplete("Microsoft library inventory refers to a different acquisition receipt.")
    if acquisition.get("files") != files:
        raise ENV.Incomplete("Microsoft library bytes differ from the admitted acquisition inventory.")
    relative = f"lib/rustlib/{policy['rust']['host']}/bin/rust-lld"
    linker_source = toolchain / relative
    linker_pin = ENV.require_admitted_binary(
        linker_source, policy["cross_toolchain"]["linker"]["binary_sha256"], "rust-lld")
    linker = work / "linker"
    args = [f"/toolchain/{relative}", "-flavor", "link", *("/libpath:/xwin/" + suffix for suffix in suffixes)]
    linker.write_text("#!/usr/bin/python3\nimport os,sys\n" + f"os.execv({args[0]!r}, {args!r} + sys.argv[1:])\n")
    linker.chmod(0o700)
    return directory, files, (args[0], linker_source, linker_pin)


def windows_libraries(root: Path, toolchain: Path, config_path: Path, work: Path,
                      inventory_path: Path | None) -> tuple[Path, dict[str, str]]:
    """Keep the SDK inventory and wrapper interface for the standalone Windows lanes."""
    directory, files, _ = prepare_windows_link(root, toolchain, config_path, work, inventory_path)
    return directory, files


def admit_advisories(receipt_path: Path | None, now: float, expected_lock: str) -> tuple[Path, dict[str, str]]:
    """Require the separately acquired complete metadata snapshot and its freshness."""
    if receipt_path is None:
        raise ENV.Incomplete("Dependency checks require a current acquisition receipt.")
    try:
        receipt = ADVISORY.read_receipt(receipt_path)
    except (OSError, ValueError) as error:
        raise ENV.Incomplete(str(error)) from error
    if receipt.get("lockfile_sha256") != expected_lock:
        raise ENV.Incomplete("Advisory/index acquisition identifies a different product lockfile.")
    acquired = receipt.get("acquired_unix")
    if (isinstance(acquired, bool) or not isinstance(acquired, (int, float))
            or not 0 <= now - acquired <= 86400):
        raise ENV.Incomplete("Advisory/index acquisition is missing, future-dated or older than 24 hours.")
    cargo_home = Path(receipt["cargo_home"])
    try:
        files = ADVISORY.inventory(cargo_home)
    except (OSError, ValueError) as error:
        raise ENV.Incomplete(str(error)) from error
    if (not files or files != receipt.get("files") or "config" in files or "config.toml" in files
            or not any(name.startswith("advisory-dbs/") for name in files)
            or not any(name.startswith("registry/index/") for name in files)):
        raise ENV.Incomplete("Advisory/index snapshot bytes or required scope are incomplete.")
    return cargo_home, files


def admit_windows_ui_tools(toolchain: Path, compiler: Path | None) -> dict[str, tuple[Path, str]]:
    """Admit the version tools and the two selected Rust LLVM library locations."""
    if compiler is None:
        raise ENV.Incomplete("Windows UI linking requires the admitted --llvm-rc executable.")
    compiler = compiler.resolve(strict=True)
    compiler_library = (compiler.parent.parent / "lib/libLLVM.so.22.1").resolve(strict=True)
    rust_tool = toolchain / READOBJ
    rust_library = toolchain / READOBJ_LIBRARY
    paths = {RC_COMPILER: compiler, RC_LIBRARY: compiler_library,
             f"/toolchain/{READOBJ}": rust_tool,
             f"/toolchain/{READOBJ_LIBRARY}": rust_library,
             LINKER_LLVM: toolchain / "lib" / rust_library.name}
    tools = {}
    for mount, path in paths.items():
        size, digest = WINDOWS_UI_TOOL_PINS[mount]
        if path.is_symlink() or not path.is_file() or path.stat().st_size != size or sha256(path) != digest:
            raise ENV.Incomplete(f"Unadmitted Windows UI version tool input: {Path(mount).name}")
        tools[mount] = (path, digest)
    if not os.access(compiler, os.X_OK) or not os.access(rust_tool, os.X_OK):
        raise ENV.Incomplete("Windows UI version tools must be executable.")
    return tools


def stage_windows_build_tools(directory: Path, tools: dict[str, tuple[Path, str]]) -> dict[str, tuple[Path, str]]:
    """Copy exact selected tools outside the child-writable tree and mount only those copies."""
    directory.mkdir(mode=0o700)
    info: os.stat_result = directory.lstat()
    if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.geteuid() or info.st_mode & 0o077:
        raise ENV.Incomplete("Windows build tools require owner-private snapshot storage.")
    staged: dict[str, tuple[Path, str]] = {}
    copies: dict[tuple[str, int], Path] = {}
    mount: str
    source: Path
    expected: str
    for mount, (source, expected) in tools.items():
        mode: int = 0o500 if "/bin/" in mount else 0o400
        key: tuple[str, int] = (expected, mode)
        # The reader and linker resolve the same LLVM bytes in distinct directories.
        if key in copies:
            staged[mount] = (copies[key], expected)
            continue
        target: Path = directory / mount.lstrip("/")
        target.parent.mkdir(parents=True, exist_ok=True)
        flags: int = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
        reader: BinaryIO
        writer: BinaryIO
        with os.fdopen(os.open(source, flags), "rb") as reader, target.open("xb") as writer:
            info = os.fstat(reader.fileno())
            # Apply the existing child per-file ceiling to the staged linker too.
            limit: int = WINDOWS_UI_TOOL_PINS.get(mount, (256 * 1024 * 1024, ""))[0]
            if not stat.S_ISREG(info.st_mode) or not 0 < info.st_size <= limit:
                raise ENV.Incomplete(f"Unadmitted Windows tool size or type: {source.name}")
            remaining: int = info.st_size
            while remaining:
                block: bytes = reader.read(min(remaining, 1024 * 1024))
                if not block:
                    break
                writer.write(block)
                remaining -= len(block)
            if remaining or reader.read(1):
                raise ENV.Incomplete(f"Windows tool extent changed during snapshotting: {source.name}")
        if sha256(target) != expected:
            raise ENV.Incomplete(f"Windows tool snapshot digest differs from the reviewed bytes: {source.name}")
        target.chmod(mode)
        staged[mount] = (target, expected)
        copies[key] = target
    return staged


def windows_tool_mounts(tools: dict[str, tuple[Path, str]], *destinations: str) -> tuple[str, ...]:
    """Bind the exact selected copies at the tools' original loader paths."""
    return tuple(argument for destination in destinations
                 for argument in ("--ro-bind", str(tools[destination][0]), destination))


def generate_windows_metadata(generation: Path, run) -> None:
    """Generate one checked source tree shared by both version-resource rows."""
    generation.mkdir(mode=0o700)
    cli = ["python3", "-I", "-B", "metadata/pipeline/__main__.py"]
    args = ["--output", "/metadata-output/tree"]
    run("ui-metadata-generate", [*cli, "generate", *args], extra=("--bind", str(generation), "/metadata-output"))
    run("ui-metadata-check", [*cli, "check", *args], extra=("--ro-bind", str(generation), "/metadata-output"))


def inspect_windows_msix(metadata_dir: Path, generation_sha256: str, run, *, source: Path,
                         linked_ui: dict, require_base_artwork: bool = False) -> dict:
    """Bind the checked source-only assembly report to both link inputs."""
    generation = metadata_dir / "generation"
    output = run(MSIX_INSPECTION_LANE, ["python3", "-I", "-B", "metadata/pipeline/__main__.py",
                                      "inspect-msix", "--output", "/metadata-output/tree"],
                 extra=("--ro-bind", str(generation), "/metadata-output", "--remount-ro", "/work"))
    spec = importlib.util.spec_from_file_location("msix_inspection_json", source / "scripts/protected_json.py")
    reader = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reader)
    try:
        inspection = reader.read(output.encode("utf-8"))
        with (generation / "tree/receipt.json").open("rb") as checked_receipt:
            receipt_bytes = checked_receipt.read(reader.MAX_BYTES + 1)
        receipt = reader.read(receipt_bytes)
    except ValueError as error:
        raise ENV.Incomplete("Invalid Windows MSIX source inspection output.") from error
    if hashlib.sha256(receipt_bytes).hexdigest() != generation_sha256:
        raise ENV.Incomplete("Windows MSIX generation receipt changed during inspection.")
    check_msix_inspection(inspection, receipt, generation_sha256, source=source)
    path = generation / "tree" / MSIX_AUTHORING_INPUT
    document = decode_msix_authoring(read_bound_msix_leaf(path, inspection["authoring_inputs"][MSIX_AUTHORING_INPUT]))
    activation = check_msix_ui_activation(document, linked_ui, generation_sha256,
                                          require_base_artwork=require_base_artwork)
    return {**inspection, "ui_activation": activation}


def check_msix_inspection(inspection: dict, receipt: dict, generation_sha256: str, *, source: Path) -> None:
    """Reconcile the source-only report with its generation and owned pending roles."""
    if (type(inspection) is not dict or set(inspection) != {
            "schema_version", "status", "distribution_route", "package_identity", "assembly_ready",
            "generation_receipt_sha256", "authoring_inputs", "roles", "pending"}
            or type(inspection["schema_version"]) is not int or inspection["schema_version"] != 1
            or inspection["status"] != "assembly-inputs-incomplete"
            or inspection["distribution_route"] != "direct-msix"
            or inspection["package_identity"] is not None or inspection["assembly_ready"] is not False
            or inspection["generation_receipt_sha256"] != generation_sha256):
        raise ENV.Incomplete("Windows MSIX source inspection differs from the checked generation or scope.")
    roles = {role: {"source_state": state, "artifact_status": "not-admitted"} for role, state in (
        ("ui", "fixture"), ("engine", "planned"), ("broker", "planned"), ("helper", "planned"))}
    if inspection["roles"] != roles:
        raise ENV.Incomplete("Windows MSIX source inspection differs from the current assembly roles.")
    # The child checked the bound receipt against all generated files and the
    # frozen source before emitting its projection. Check the projected fields.
    with (source / "metadata/product.toml").open("rb") as product:
        pending = [{key: item[key] for key in ("path", "owner", "required_stage")}
                   for item in tomllib.load(product)["pending"]]
    authoring = {}
    try:
        for row in receipt["files"]:
            path = row["path"]
            if path not in MSIX_ALLOWED_AUTHORING:
                continue
            if path in authoring:
                raise ENV.Incomplete("Duplicate Windows MSIX generation receipt authoring path.")
            authoring[path] = row["sha256"]
    except (KeyError, TypeError) as error:
        raise ENV.Incomplete("Invalid Windows MSIX generation receipt file inventory.") from error
    if not MSIX_REQUIRED_AUTHORING <= authoring.keys() or inspection["authoring_inputs"] != authoring:
        raise ENV.Incomplete("Windows MSIX source inspection differs from the checked authoring inputs.")
    if inspection["pending"] != sorted(pending, key=lambda item: item["path"]):
        raise ENV.Incomplete("Windows MSIX source inspection differs from the pending release inputs.")


def check_msix_ui_activation(document: dict, linked_ui: dict, generation_sha256: str, *,
                             require_base_artwork: bool) -> dict:
    """Bind checked partial UI authoring to the version and execution resource report."""
    expected_activation = {
        "role": "ui", "executable": f"{PACKAGE}.exe", "application_id": None,
        "runtime_behavior": "packagedClassicApp", "trust_level": "mediumIL",
        "visual_elements": {
            "Square44x44Logo": "Assets/Square44x44Logo.png",
            "Square150x150Logo": "Assets/Square150x150Logo.png",
        },
    }
    if (type(document.get("schema_version")) is not int
            or document["schema_version"] != 3
            or document.get("ui_activation") != expected_activation):
        raise ENV.Incomplete("Windows MSIX UI activation differs from the checked application profile.")
    manifest = linked_ui.get("execution_manifest", {})
    if (linked_ui.get("generation_sha256") != generation_sha256
            or linked_ui.get("strings", {}).get("OriginalFilename") != expected_activation["executable"]
            or manifest.get("level") != "asInvoker" or manifest.get("ui_access") is not False):
        raise ENV.Incomplete("Windows MSIX UI activation differs from the linked UI role or generation.")
    if require_base_artwork:
        require_msix_base_artwork_paths(document)
    return expected_activation


def _artwork_identity(info: os.stat_result) -> tuple[int, ...]:
    """Bind a regular input or its containing directory for one bounded read."""
    return (info.st_dev, info.st_ino, info.st_mode, info.st_nlink, info.st_size,
            info.st_mtime_ns, info.st_ctime_ns)


def _matches_msix_export_path(path: Path, resolved: Path, staging_descriptor: int | None) -> bool:
    """Reject aliases below the exact held export directory, or in the whole input path."""
    if staging_descriptor is None:
        return path == resolved
    staging = Path(f"/proc/self/fd/{staging_descriptor}")
    if not path.is_relative_to(staging):
        return False
    relative = path.relative_to(staging)
    return ".." not in relative.parts and resolved == staging.resolve(strict=True) / relative


def read_bound_msix_leaf(path: Path, expected_sha256: str, *,
                         staging_descriptor: int | None = None,
                         maximum: int = MSIX_AUTHORING_MAX_BYTES) -> bytes:
    """Hold one bounded single-link leaf and confirm its original path and bytes."""
    custody_error = "Windows MSIX authoring differs from the checked generation."
    flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
    try:
        if not _matches_msix_export_path(path, path.resolve(strict=True), staging_descriptor):
            raise ENV.Incomplete(custody_error)
    except (OSError, RuntimeError) as error:
        raise ENV.Incomplete(custody_error) from error
    try:
        with os.fdopen(os.open(path, flags), "rb") as stream:
            before = os.fstat(stream.fileno())
            if (not stat.S_ISREG(before.st_mode) or before.st_nlink != 1
                    or not 0 < before.st_size <= maximum):
                raise ENV.Incomplete(custody_error)
            raw = stream.read(maximum + 1)
            identity = _artwork_identity(before)
            if (len(raw) != before.st_size or hashlib.sha256(raw).hexdigest() != expected_sha256
                    or _artwork_identity(os.fstat(stream.fileno())) != identity
                    or _artwork_identity(path.stat(follow_symlinks=False)) != identity):
                raise ENV.Incomplete(custody_error)
    except OSError as error:
        raise ENV.Incomplete(custody_error) from error
    return raw


def read_bound_ui_entry_map(path: Path, entry: dict[str, Any], image_pin: str, *,
                            staging_descriptor: int | None = None) -> bytes:
    """Reopen only the recorded map for the exact guarded UI image."""
    error_message: str = "Windows UI baseline link evidence differs from the checked link."
    if type(entry) is not dict or entry.get("image_sha256") != image_pin:
        raise ENV.Incomplete(error_message)
    map_pin: object = entry.get("map_sha256")
    if type(map_pin) is not str or re.fullmatch(r"[0-9a-f]{64}", map_pin) is None:
        raise ENV.Incomplete(error_message)
    try:
        return read_bound_msix_leaf(path, map_pin, staging_descriptor=staging_descriptor,
                                    maximum=UI_ENTRY_MAP_MAX_BYTES)
    except ENV.Incomplete as error:
        raise ENV.Incomplete(error_message) from error


def unique_msix_json_fields(pairs):
    """Retain only one decoded member for each JSON object key."""
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("Duplicate MSIX authoring field.")
        result[key] = value
    return result


def decode_msix_authoring(raw: bytes) -> dict:
    """Read the held authoring leaf without narrowing its admitted descriptive text."""

    def invalid_number(_value):
        raise ValueError("Unselected MSIX authoring number.")

    try:
        document = json.loads(raw.decode("utf-8"), object_pairs_hook=unique_msix_json_fields,
                              parse_float=invalid_number, parse_constant=invalid_number)
    except (RecursionError, ValueError) as error:
        raise ENV.Incomplete("Invalid Windows MSIX UI activation authoring.") from error
    if type(document) is not dict:
        raise ENV.Incomplete("Invalid Windows MSIX UI activation authoring.")
    return document


def require_msix_base_artwork_paths(document: dict) -> str:
    """Match the three export roles to the exact already checked authoring leaf."""
    custody_error = "Windows MSIX base artwork authoring differs from the checked generation."
    rows = document.get("fields")
    expected = {}
    for name, (role, _, _, _) in MSIX_BASE_ARTWORK.items():
        key, status = MSIX_ARTWORK_FIELDS[role]
        expected[key] = ("Assets/" + name, status)
    if type(rows) is not list:
        raise ENV.Incomplete(custody_error)
    found = {}
    for row in rows:
        if type(row) is not dict or type(row.get("key")) is not str:
            raise ENV.Incomplete(custody_error)
        key = row["key"]
        if key in expected:
            if key in found or (row.get("value"), row.get("status")) != expected[key]:
                raise ENV.Incomplete("Windows MSIX base artwork role or package path differs.")
            found[key] = row["value"]
    if found.keys() != expected.keys():
        raise ENV.Incomplete("Windows MSIX base artwork authoring rows are incomplete.")
    return found[MSIX_ARTWORK_FIELDS["Properties/Logo"][0]]


def render_msix_properties(document: dict) -> bytes:
    """Encode checked display data, package integrity and Trusted Launch authoring."""
    logo = require_msix_base_artwork_paths(document)
    properties = ET.Element("Properties", {
        "xmlns": MSIX_FOUNDATION_NAMESPACE, "xmlns:uap10": MSIX_UAP10_NAMESPACE,
        "xmlns:trustedlaunch": MSIX_TRUSTED_LAUNCH_NAMESPACE,
    })
    for name, value in (
        ("DisplayName", document.get("display_name")),
        ("PublisherDisplayName", document.get("publisher_display_name")),
        ("Description", document.get("description")),
    ):
        if type(value) is not str:
            raise ENV.Incomplete(f"Windows MSIX Properties {name} requires literal text.")
        ET.SubElement(properties, name).text = value
    ET.SubElement(properties, "Logo").text = logo.replace("/", "\\")
    integrity = ET.SubElement(properties, "uap10:PackageIntegrity")
    ET.SubElement(integrity, "uap10:Content", {"Enforcement": "on"})
    ET.SubElement(properties, "trustedlaunch:TrustedLaunch").text = "true"
    ET.indent(properties, space="  ")
    return MSIX_XML_DECLARATION + b"\n" + ET.tostring(properties, encoding="utf-8") + b"\n"


def verify_msix_properties(raw: bytes, *, source: Path, generation_receipt: bytes) -> None:
    """Read the current direct-MSIX fragment against checked source and required protections."""
    product_path = "metadata/product.toml"
    inputs = decode_msix_authoring(generation_receipt).get("inputs")
    if type(inputs) is not list:
        raise ENV.Incomplete("Windows MSIX Properties lacks its checked product source.")
    product_digests = [row.get("sha256") for row in inputs
                       if type(row) is dict and row.get("path") == product_path]
    if len(product_digests) != 1:
        raise ENV.Incomplete("Windows MSIX Properties lacks its unique checked product source.")
    product_bytes = read_bound_msix_leaf(source / product_path, product_digests[0])
    try:
        product = tomllib.loads(product_bytes.decode("utf-8"))["product"]
        expected = {"DisplayName": product["name"], "PublisherDisplayName": product["publisher"],
                    "Description": product["description"], "Logo": MSIX_PACKAGE_LOGO}
    except (KeyError, TypeError, ValueError) as error:
        raise ENV.Incomplete("Windows MSIX Properties lacks its checked product fields.") from error

    refused = "Windows MSIX Properties differs from the checked display, package integrity or Trusted Launch contract."
    declaration, _, body = raw.partition(b"\n")
    if (not 0 < len(raw) <= MSIX_AUTHORING_MAX_BYTES or declaration != MSIX_XML_DECLARATION
            or b"<!" in body or b"<?" in body):
        raise ENV.Incomplete(refused)
    try:
        raw.decode("utf-8")
        properties = ET.fromstring(raw)
    except (ET.ParseError, ValueError) as error:
        raise ENV.Incomplete(refused) from error
    foundation = "{" + MSIX_FOUNDATION_NAMESPACE + "}"
    uap10 = "{" + MSIX_UAP10_NAMESPACE + "}"
    leaves = {foundation + name: value for name, value in expected.items()}
    # This current profile requires regeneration of fragments that predate Trusted Launch.
    leaves["{" + MSIX_TRUSTED_LAUNCH_NAMESPACE + "}TrustedLaunch"] = "true"
    required = leaves.keys() | {uap10 + "PackageIntegrity"}
    children = {child.tag: child for child in properties}
    if (properties.tag != foundation + "Properties" or properties.attrib
            or (properties.text or "").strip(" \t\r\n")
            or len(properties) != len(required) or children.keys() != required
            or any((child.tail or "").strip(" \t\r\n") for child in properties)):
        raise ENV.Incomplete(refused)
    for name, value in leaves.items():
        leaf = children[name]
        if leaf.attrib or len(leaf) or leaf.text != value:
            raise ENV.Incomplete(refused)
    # The foundation schema uses ST_DisplayName for both names and
    # ST_Description for the description. This fragment uses literal text.
    for name, limit in (("DisplayName", 256), ("PublisherDisplayName", 256), ("Description", 2048)):
        value = expected[name]
        if (type(value) is not str or value != value.strip(" \t\r\n")
                or re.search(r"[\x00-\x1f]", value)
                or not 1 <= len(value) <= limit
                or value.lower().startswith("ms-resource:")):
            raise ENV.Incomplete(f"Windows MSIX Properties {name} requires bounded literal text.")
    integrity = children[uap10 + "PackageIntegrity"]
    if integrity.attrib or (integrity.text or "").strip(" \t\r\n") or len(integrity) != 1:
        raise ENV.Incomplete(refused)
    content = integrity[0]
    if (content.tag != uap10 + "Content" or content.attrib != {"Enforcement": "on"}
            or content.text or len(content) or (content.tail or "").strip(" \t\r\n")):
        raise ENV.Incomplete(refused)


def msix_authoring_digests(inspection: dict) -> dict[str, str]:
    """Select the exact private assembly files from one checked inspection."""
    authoring = inspection["authoring_inputs"]
    if not MSIX_REQUIRED_AUTHORING <= authoring.keys() <= MSIX_ALLOWED_AUTHORING:
        raise ENV.Incomplete("Unexpected Windows MSIX authoring input inventory.")
    return {**authoring, "receipt.json": inspection["generation_receipt_sha256"]}


def read_msix_authoring_snapshot(tree: Path, inspection: dict) -> dict[str, bytes]:
    """Keep only the inspected private authoring inputs and their generation receipt."""
    return {name: read_bound_msix_leaf(tree / name, digest)
            for name, digest in msix_authoring_digests(inspection).items()}


def export_msix_authoring(frozen: dict[str, bytes], output: Path, *, inspection: dict,
                          staging_descriptor: int | None = None,
                          properties_source: Path | None = None) -> dict:
    """Write held assembly source into the private build export and verify readback."""
    expected = msix_authoring_digests(inspection)
    if frozen.keys() != expected.keys():
        raise ENV.Incomplete("Unexpected Windows MSIX authoring export inventory.")
    if any(type(data) is not bytes or not 0 < len(data) <= MSIX_AUTHORING_MAX_BYTES
           or hashlib.sha256(data).hexdigest() != expected[name] for name, data in frozen.items()):
        raise ENV.Incomplete("Invalid retained Windows MSIX authoring snapshot.")
    if staging_descriptor is None:
        info = output.stat(follow_symlinks=False)
        matches = output.resolve(strict=True) == output
    else:
        info = os.fstat(staging_descriptor)
        matches = output == Path(f"/proc/self/fd/{staging_descriptor}")
    if not matches or not stat.S_ISDIR(info.st_mode) or info.st_uid != os.geteuid() or info.st_mode & 0o077:
        raise ENV.Incomplete("Windows MSIX authoring requires an owner-private export directory.")
    payload = frozen
    if properties_source is not None:
        fragment = render_msix_properties(decode_msix_authoring(frozen[MSIX_AUTHORING_INPUT]))
        payload = {**frozen, MSIX_PROPERTIES_FRAGMENT: fragment}
        expected[MSIX_PROPERTIES_FRAGMENT] = hashlib.sha256(fragment).hexdigest()
    export = output / "msix-authoring"
    files = {}
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC
    with tempfile.TemporaryDirectory(prefix=".msix-authoring-", dir=output) as temporary:
        stage = Path(temporary) / export.name
        stage.mkdir(mode=0o700)
        (stage / "authoring").mkdir(mode=0o700)
        for name, data in payload.items():
            path = stage / name
            with os.fdopen(os.open(path, flags, mode=0o600), "wb") as stream:
                stream.write(data)
            digest = expected[name]
            observed = read_bound_msix_leaf(path, digest, staging_descriptor=staging_descriptor)
            if observed != data:
                raise ENV.Incomplete("Windows MSIX authoring changed during export.")
            if name == MSIX_PROPERTIES_FRAGMENT:
                assert properties_source is not None
                verify_msix_properties(observed, source=properties_source, generation_receipt=frozen["receipt.json"])
            files[name] = {"bytes": len(data), "sha256": digest}
        if export.exists() or export.is_symlink():
            raise ENV.Incomplete("Windows MSIX authoring export already exists.")
        stage.rename(export)
    return {"scope": "private-assembly-source", "export_root": "msix-authoring", "files": files}


def _msix_artwork_selection(names: Set[str]) -> tuple[str, dict[str, tuple[str, int, int, str]]]:
    """Select the exact base roster or the complete pinned qualified roster."""
    if names == MSIX_BASE_ARTWORK.keys():
        return "candidate-base-resources", MSIX_BASE_ARTWORK
    qualified = MSIX_BASE_ARTWORK | MSIX_VARIANT_ARTWORK
    if names == qualified.keys():
        return "candidate-scale-and-target-resources", qualified
    raise ENV.Incomplete("Windows MSIX artwork differs from the selected input set.")


def read_msix_artwork(directory: Path, *, staging_descriptor: int | None = None) -> dict[str, bytes]:
    """Hold the exact pinned base or qualified artwork snapshot from an explicit directory."""
    directory = directory.absolute()
    refused = "Windows MSIX artwork differs from the selected input set."
    try:
        resolved = Path(os.path.realpath(directory, strict=True))
        if not _matches_msix_export_path(directory, resolved, staging_descriptor):
            raise ENV.Incomplete(refused)
        dir_flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
        parent = os.open(directory, dir_flags)
        try:
            initial = _artwork_identity(os.fstat(parent))
            allowed = MSIX_BASE_ARTWORK.keys() | MSIX_VARIANT_ARTWORK.keys()
            seen = set()
            with os.scandir(parent) as entries:
                for entry in entries:
                    if entry.name in seen or entry.name not in allowed:
                        raise ENV.Incomplete(refused)
                    seen.add(entry.name)
            _, selected = _msix_artwork_selection(seen)
            frozen = {}
            flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
            for name, (_, _, length, expected) in selected.items():
                with os.fdopen(os.open(name, flags, dir_fd=parent), "rb") as stream:
                    before = os.fstat(stream.fileno())
                    if not stat.S_ISREG(before.st_mode) or before.st_nlink != 1 or before.st_size != length:
                        raise ENV.Incomplete(refused)
                    data = stream.read(length + 1)
                    if (len(data) != length or hashlib.sha256(data).hexdigest() != expected
                            or _artwork_identity(before) != _artwork_identity(os.fstat(stream.fileno()))
                            or _artwork_identity(before) != _artwork_identity(
                                os.stat(name, dir_fd=parent, follow_symlinks=False))):
                        raise ENV.Incomplete(refused)
                frozen[name] = data
            if initial != _artwork_identity(os.stat(directory, follow_symlinks=False)):
                raise ENV.Incomplete(refused)
            return frozen
        finally:
            os.close(parent)
    except OSError as error:
        raise ENV.Incomplete(refused) from error


def export_msix_artwork(frozen: dict[str, bytes], output: Path, *,
                       staging_descriptor: int | None = None) -> dict:
    """Publish held image bytes in the owned export and verify the same selected roster."""
    scope, selected = _msix_artwork_selection(frozen.keys())
    if any(len(data) != selected[name][2]
           or hashlib.sha256(data).hexdigest() != selected[name][3]
           for name, data in frozen.items()):
        raise ENV.Incomplete("Invalid retained Windows MSIX artwork snapshot.")
    assembly_inputs = output / "msix-inputs"
    assembly_inputs.mkdir(mode=0o700)
    assets = assembly_inputs / "Assets"
    assets.mkdir(mode=0o700)
    for name, data in frozen.items():
        with (assets / name).open("xb") as stream:
            stream.write(data)
    if read_msix_artwork(assets, staging_descriptor=staging_descriptor) != frozen:
        raise ENV.Incomplete("Windows MSIX artwork changed during export.")
    return msix_artwork_export_record(scope, selected)


def msix_artwork_export_record(scope: str, selected: dict) -> dict:
    """Describe verified artwork using the exact selected pin and role for each name."""
    return {"scope": scope, "export_root": "msix-inputs",
            "files": {"Assets/" + name: {"manifest_field": field, "width": dimension,
                                         "height": dimension, "bytes": length, "sha256": digest}
                      for name, (field, dimension, length, digest) in selected.items()}}


def _msix_handoff_inventory(descriptor: int, leaves: set[str]) -> dict[str, tuple[int, ...]]:
    """Read only the expected flat and nested names and hold their file identities."""
    directories = {".": set()}
    for name in leaves:
        relative = Path(name)
        for parent, child in zip(reversed(relative.parents), relative.parts):
            directories.setdefault(parent.as_posix(), set()).add(child)
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
    observed = {}
    refused = "Unexpected Windows MSIX handoff file or directory inventory."
    for relative, expected in sorted(directories.items()):
        child = os.open(relative, flags, dir_fd=descriptor)
        try:
            observed[relative] = _artwork_identity(os.fstat(child))
            with os.scandir(child) as entries:
                missing = set(expected)
                for entry in entries:
                    name = (Path(relative) / entry.name).as_posix()
                    info = entry.stat(follow_symlinks=False)
                    if entry.name not in missing:
                        raise ENV.Incomplete(refused)
                    missing.remove(entry.name)
                    if name in leaves:
                        if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
                            raise ENV.Incomplete(refused)
                        observed[name] = _artwork_identity(info)
                    elif not stat.S_ISDIR(info.st_mode):
                        raise ENV.Incomplete(refused)
                if missing:
                    raise ENV.Incomplete(refused)
        finally:
            os.close(child)
    return observed


def _msix_handoff_image(artifact: dict, versioninfo: dict, role: str, state: str,
                        name: str, generation: str) -> tuple[str, str]:
    """Select one fixed linked image and its private symbol leaf from the pinned receipt."""
    if (artifact["component_role"] != role or artifact["source_state"] != state
            or artifact["profile"] != "release" or artifact["target"] != ENV.WINDOWS_TARGET
            or artifact["sha256"] != versioninfo["image_sha256"]
            or versioninfo["generation_sha256"] != generation
            or versioninfo["strings"]["OriginalFilename"] != name + ".exe"
            or Path(artifact["path"]).name != name + ".exe"):
        raise ENV.Incomplete("Windows MSIX handoff image role or generation differs.")
    symbol = Path(artifact["symbols"]["path"]).name
    if symbol not in {name + ".pdb", name.replace("-", "_") + ".pdb"}:
        raise ENV.Incomplete("Windows MSIX handoff symbol role differs.")
    return name + ".exe", symbol


def _read_msix_handoff_export(source: Path, directory: Path, descriptor: int, receipt_sha256: str) -> dict:
    """Inspect one retained handoff without promoting any leaf to shipping payload."""
    anchor = Path(f"/proc/self/fd/{descriptor}")
    receipt_bytes = read_bound_msix_leaf(anchor / "verification.json", receipt_sha256,
                                          staging_descriptor=descriptor, maximum=MSIX_HANDOFF_RECEIPT_MAX_BYTES)
    report = json.loads(receipt_bytes.decode("utf-8"), object_pairs_hook=unique_msix_json_fields)
    if report["status"] != "passed" or report["native_windows_execution"] != "not performed":
        raise ENV.Incomplete("Windows MSIX handoff does not identify a successful cross-build export.")
    inspection = report["msix_assembly_inputs"]
    if type(inspection) is not dict:
        raise ENV.Incomplete("Windows MSIX handoff assembly inputs must be an object.")
    inspection = inspection.copy()
    claimed_activation = inspection.pop("ui_activation")
    claimed_artwork = inspection.pop("base_artwork")
    claimed_authoring = inspection.pop("authoring_export")
    generation = inspection["generation_receipt_sha256"]
    authoring_names = {*msix_authoring_digests(inspection), MSIX_PROPERTIES_FRAGMENT}
    if (claimed_authoring["scope"] != "private-assembly-source"
            or claimed_authoring["export_root"] != "msix-authoring"
            or claimed_authoring["files"].keys() != authoring_names):
        raise ENV.Incomplete("Windows MSIX handoff private authoring inventory differs.")

    artwork_names = {Path(name).name for name in claimed_artwork["files"]}
    artwork_scope, artwork_pins = _msix_artwork_selection(artwork_names)
    if (claimed_artwork != msix_artwork_export_record(artwork_scope, artwork_pins)
            or any(type(row[key]) is not int for row in claimed_artwork["files"].values()
                   for key in ("width", "height", "bytes"))):
        raise ENV.Incomplete("Windows MSIX handoff artwork differs from its pinned roster.")
    artwork_paths = {"msix-inputs/" + name for name in claimed_artwork["files"]}
    authoring_paths = {"msix-authoring/" + name for name in authoring_names}

    images = report["artifact"]
    ui, ui_symbols = _msix_handoff_image(images, report["ui_versioninfo"],
                                         "ui", "fixture", PACKAGE, generation)
    observer, observer_symbols = _msix_handoff_image(images["engine"], report["observe_versioninfo"],
                                                     "observer", "experiment", "nevrotick-observe", generation)
    artifacts = {"verification.json": "private-receipt", ui: "unadmitted-ui-image",
                 observer: "excluded-observe-image", ui_symbols: "private-symbols",
                 observer_symbols: "private-symbols",
                 **dict.fromkeys(authoring_paths, "private-authoring"),
                 **dict.fromkeys(artwork_paths, "unadmitted-artwork")}
    entry: dict[str, Any] | None = report.get("ui_preentry")
    if entry is not None and type(entry) is not dict:
        raise ENV.Incomplete("Windows MSIX handoff UI entry record must be an object.")
    # Earlier guarded reports stored the raw map separately.
    if entry is not None and "map_artifact" in entry:
        if entry["map_artifact"] != UI_ENTRY_MAP_LEAF:
            raise ENV.Incomplete("Windows MSIX handoff UI link map artifact differs.")
        artifacts[UI_ENTRY_MAP_LEAF] = "private-ui-link-map"
    else:
        entry = None
    initial = _msix_handoff_inventory(descriptor, set(artifacts))
    # The roster bookends include all files. Rebind the receipt to the first bookend
    # as it supplied the selection read before the roster could be established.
    if read_bound_msix_leaf(anchor / "verification.json", receipt_sha256, staging_descriptor=descriptor,
                            maximum=MSIX_HANDOFF_RECEIPT_MAX_BYTES) != receipt_bytes:
        raise ENV.Incomplete("Windows MSIX handoff receipt changed during inspection.")

    verified_digests = {"verification.json": receipt_sha256}
    pe = windows_pe_reader(source)
    observed_ui: dict[str, object] = {}
    observed_imports: dict[str, dict[str, list[str]]] = {}
    for role, image, symbol, subsystem, claimed in (
        ("ui", ui, ui_symbols, "gui", images),
        ("observe", observer, observer_symbols, "console", images["engine"]),
    ):
        image_record: tuple[bytes, str]
        symbols_record: dict[str, str | int]
        image_record, _, symbols_record = pe.read_export_pair(
            anchor / image, anchor / symbol, subsystem=subsystem)
        claimed_symbols = {key: claimed["symbols"][key] for key in ("sha256", "guid", "age")}
        if (image_record[1] != claimed["sha256"] or type(claimed_symbols["age"]) is not int
                or symbols_record != claimed_symbols):
            raise ENV.Incomplete("Windows MSIX handoff PE or PDB differs from its pinned receipt.")
        try:
            imports: dict[str, list[str]] = pe.static_import_modules(image_record[0], subsystem=subsystem)
        except ValueError as error:
            raise ENV.Incomplete(f"Windows MSIX handoff {role} static import readback refused: {error}") from error
        claimed_versioninfo: dict[str, Any] = report[role + "_versioninfo"]
        claimed_imports: object = claimed_versioninfo.get("static_imports")
        if imports != claimed_imports:
            raise ENV.Incomplete(f"Windows MSIX handoff {role} static imports differ from the pinned receipt.")
        observed_imports[role] = imports
        declarations: dict[str, str | int] = pe.pe_image_declarations(
            image_record[0], anchor / image, subsystem=subsystem, imports=True)
        claimed_declarations: object = claimed_versioninfo.get("pe_declarations")
        if (type(claimed_declarations) is not dict or claimed_declarations != declarations
                or any(type(claimed_declarations[key]) is not type(value)
                       for key, value in declarations.items())):
            raise ENV.Incomplete(f"Windows MSIX handoff {role} PE declarations differ from the pinned receipt.")
        if image == ui:
            if entry is not None:
                map_bytes: bytes = read_bound_ui_entry_map(
                    anchor / UI_ENTRY_MAP_LEAF, entry, image_record[1], staging_descriptor=descriptor)
                try:
                    baseline: dict[str, object] = pe.ui_preentry_declarations(
                        image_record[0], map_bytes.decode("utf-8"), object_sha256=entry.get("object_sha256"))
                except ValueError as error:
                    raise ENV.Incomplete(f"Windows MSIX handoff UI link map readback refused: {error}") from error
                if any(type(entry.get(key)) is not type(value) or entry[key] != value
                       for key, value in baseline.items()):
                    raise ENV.Incomplete("Windows MSIX handoff UI entry differs from the pinned receipt.")
                verified_digests[UI_ENTRY_MAP_LEAF] = entry["map_sha256"]
            reader: ModuleType = windows_versioninfo_reader(source)
            version: VersionInfo
            manifest: UiExecutionManifest
            try:
                version, manifest = reader.decode_ui_resource_payloads(*pe.ui_resource_payloads(image_record[0]))
            except ValueError as error:
                raise ENV.Incomplete(f"Windows MSIX handoff UI resource readback refused: {error}") from error
            claimed_ui: dict[str, Any] = claimed_versioninfo
            claimed_manifest: object = claimed_ui["execution_manifest"]
            claimed_fixed: object = claimed_ui.get("fixed")
            claimed_translation: object = claimed_ui.get("translation")
            if (type(claimed_manifest) is not dict or claimed_manifest != manifest
                    or claimed_manifest["ui_access"] is not manifest["ui_access"]
                    or version["strings"] != claimed_ui["strings"]
                    or type(claimed_fixed) is not list or claimed_fixed != list(version["fixed"])
                    or any(type(word) is not int for word in claimed_fixed)
                    or type(claimed_translation) is not list or claimed_translation != list(version["translation"])
                    or any(type(word) is not int for word in claimed_translation)):
                raise ENV.Incomplete("Windows MSIX handoff UI resources differ from the pinned receipt.")
            observed_ui = {"generation_sha256": claimed_ui["generation_sha256"],
                           "strings": version["strings"], "execution_manifest": manifest}
        verified_digests[image] = image_record[1]
        verified_digests[symbol] = symbols_record["sha256"]

    authoring = {}
    for name, field in claimed_authoring["files"].items():
        content = read_bound_msix_leaf(anchor / "msix-authoring" / name, field["sha256"],
                                        staging_descriptor=descriptor)
        if type(field["bytes"]) is not int or field != {"bytes": len(content), "sha256": field["sha256"]}:
            raise ENV.Incomplete("Windows MSIX handoff private authoring sizes differ.")
        authoring[name] = content
        verified_digests["msix-authoring/" + name] = field["sha256"]
    for name, digest in msix_authoring_digests(inspection).items():
        if claimed_authoring["files"][name]["sha256"] != digest:
            raise ENV.Incomplete("Windows MSIX handoff private authoring generation differs.")
    source_receipt = authoring["receipt.json"]
    check_msix_inspection(inspection, decode_msix_authoring(source_receipt), generation, source=source)
    document = decode_msix_authoring(authoring[MSIX_AUTHORING_INPUT])
    activation = check_msix_ui_activation(document, observed_ui, generation, require_base_artwork=True)
    if (activation != claimed_activation or document["identity"] is not None
            or document["complete_manifest"] is not False or document["distribution_route"] != "direct-msix"):
        raise ENV.Incomplete("Windows MSIX handoff activation or assembly status differs.")
    verify_msix_properties(authoring[MSIX_PROPERTIES_FRAGMENT], source=source, generation_receipt=source_receipt)
    read_msix_artwork(anchor / "msix-inputs/Assets", staging_descriptor=descriptor)
    verified_digests.update({"msix-inputs/" + name: field["sha256"]
                             for name, field in claimed_artwork["files"].items()})
    if (initial != _msix_handoff_inventory(descriptor, set(artifacts))
            or initial["."] != _artwork_identity(directory.stat(follow_symlinks=False))):
        raise ENV.Incomplete("Windows MSIX handoff tree changed during inspection.")
    return {
        "schema_version": 1, "status": "handoff-verified", "assembly_ready": False,
        "distribution_route": "direct-msix", "handoff_receipt_sha256": receipt_sha256,
        "generation_receipt_sha256": generation, "package_identity": None,
        "roles": inspection["roles"], "pending": inspection["pending"],
        "ui_application_id": activation["application_id"],
        "static_imports": observed_imports,
        "product_dependency_and_resource_closure": "not-admitted",
        "product_platform_and_activation_profile": "not-admitted",
        "files": {name: {"disposition": value, "bytes": initial[name][4],
                         "sha256": verified_digests[name]}
                  for name, value in sorted(artifacts.items())},
    }


def inspect_msix_handoff(source: Path, directory: Path, receipt_sha256: str) -> dict:
    """Read only a held, private Windows export selected by an independent receipt pin."""
    if not re.fullmatch(r"[0-9a-f]{64}", receipt_sha256):
        raise ENV.Incomplete("Windows MSIX handoff requires the pinned lowercase receipt SHA-256.")
    directory = directory.absolute()
    if directory.resolve(strict=True) != directory:
        raise ENV.Incomplete("Windows MSIX handoff path must name its original directory.")
    descriptor = os.open(directory, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        info = os.fstat(descriptor)
        if (info.st_uid != os.geteuid() or info.st_mode & 0o077
                or not os.path.samestat(info, directory.stat(follow_symlinks=False))):
            raise ENV.Incomplete("Windows MSIX handoff requires an owner-private export directory.")
        return _read_msix_handoff_export(source, directory, descriptor, receipt_sha256)
    finally:
        os.close(descriptor)


def compile_windows_versioninfo(directory: Path, role: str, tools: dict, run) -> tuple[Path, dict, dict]:
    """Compile the fixed role's checked input with an isolated output directory."""
    if role not in {"ui", "observe"}:
        raise ENV.Incomplete("Unsupported Windows version resource role.")
    generation = directory / "generation"
    compiled = directory / f"compiled-{role}"
    compiled.mkdir(mode=0o700)
    expected = json.loads(run(f"{role}-versioninfo-expectation",
                              ["python3", "-I", "-B", "scripts/windows_versioninfo_expectation.py", role]))
    rc = generation / f"tree/rc/{role}.rc"
    if rc.is_symlink() or not rc.is_file() or not 0 < rc.stat().st_size <= 64 * 1024:
        raise ENV.Incomplete(f"Missing or oversized checked {role} resource source.")
    inputs = {"rc_sha256": sha256(rc), "generation_sha256": sha256(generation / "tree/receipt.json")}
    mounts = ("--ro-bind", str(rc), "/metadata-input/version.rc", "--bind", str(compiled), "/metadata-build",
              "--remount-ro", "/work", *windows_tool_mounts(tools, RC_COMPILER, RC_LIBRARY))
    run(f"{role}-versioninfo-compile", [RC_COMPILER, "/no-preprocess", "/C", "65001", "/FO",
                                      "/metadata-build/version.res", "--", "/metadata-input/version.rc"], extra=mounts)
    resource = compiled / "version.res"
    if resource.is_symlink() or not resource.is_file() or not 0 < resource.stat().st_size <= 4096:
        raise ENV.Incomplete(f"Missing or oversized compiled {role} version resource.")
    if inputs != {"rc_sha256": sha256(rc), "generation_sha256": sha256(generation / "tree/receipt.json")}:
        raise ENV.Incomplete(f"Checked {role} metadata changed during compilation.")
    inputs["res_sha256"] = sha256(resource)
    return resource, expected, inputs


def read_windows_versioninfo(work: Path, source: Path, role: str, expected: dict, tools: dict, run) -> dict:
    """Check PE declarations, resources and imports against the selected linked role."""
    binaries = {"ui": PACKAGE, "observe": "nevrotick-observe"}
    if role not in binaries:
        raise ENV.Incomplete("Unsupported Windows version resource role.")
    reader: ModuleType = windows_versioninfo_reader(source)
    relative = f"target/{ENV.WINDOWS_TARGET}/release/{binaries[role]}.exe"
    binary = work / relative
    pe = windows_pe_reader(source)
    declarations = pe.pe_readback(binary, subsystem="gui" if role == "ui" else "console", imports=True)
    identity = declarations["image_sha256"]
    mounts = ("--remount-ro", "/work", *windows_tool_mounts(tools, f"/toolchain/{READOBJ}",
                                                            f"/toolchain/{READOBJ_LIBRARY}"))
    report = run(f"{role}-versioninfo-readback", [f"/toolchain/{READOBJ}", "--file-headers",
                                                "--coff-resources", f"/work/{relative}"],
                 extra=mounts)
    execution_manifest = None
    if role == "ui":
        actual, execution_manifest = reader.decode_ui_resources(report)
    else:
        actual = reader.decode(report, role=role)
    observed = {"fixed": list(actual["fixed"]), "strings": actual["strings"],
                "translation": list(actual["translation"])}
    if observed != expected:
        raise ENV.Incomplete(f"Linked {role} version resource differs from the checked source facts.")
    imports = run(f"{role}-static-import-readback", [f"/toolchain/{READOBJ}", "--coff-imports",
                                                    f"/work/{relative}"],
                  extra=mounts)
    static_imports = reader.decode_imports(imports, image=f"/work/{relative}")
    expected_counts = {kind: declarations[f"{kind}_import_descriptors"] for kind in ("direct", "delay")}
    if {kind: len(modules) for kind, modules in static_imports.items()} != expected_counts:
        raise ENV.Incomplete(f"Linked {role} import report differs from the image's descriptor counts.")
    if sha256(binary) != identity:
        raise ENV.Incomplete(f"Linked {role} image changed during static readback.")
    record = {"image_sha256": identity, "pe_declarations": declarations,
              "static_imports": static_imports, **observed}
    if execution_manifest is not None:
        record["execution_manifest"] = execution_manifest
    return record


def admit_ui_entry_tools() -> None:
    """Check the fixed baseline Clang and its two LLVM libraries."""
    for path, digest in UI_ENTRY_TOOL_PINS.items():
        if path.is_symlink() or not path.is_file() or sha256(path) != digest:
            raise ENV.Incomplete(f"Unadmitted Windows UI baseline compiler input: {path.name}")
    if not os.access(UI_ENTRY_CLANG, os.X_OK):
        raise ENV.Incomplete("Windows UI baseline compiler must be executable.")


def compile_ui_entry(directory: Path, run) -> tuple[Path, str]:
    """Exercise the entry's selector, then compile its freestanding Windows object."""
    compiled: Path = directory / "compiled-ui-entry"
    compiled.mkdir(mode=0o700)
    flags: list[str] = [str(UI_ENTRY_CLANG), "--no-default-config", "-std=c17", "-O2",
                        "-Wall", "-Wextra", "-Werror", "-Wpedantic", "-march=x86-64"]
    output_mounts: tuple[str, ...] = ("--bind", str(compiled), "/baseline-build", "--remount-ro", "/work")
    run("ui-baseline-entry-controls-compile", [*flags, "--target=" + HOST_TARGET,
        "/work/fixture/" + UI_ENTRY_CONTROL_SOURCE, "-o", "/baseline-build/entry-controls"], extra=output_mounts)
    run("ui-baseline-entry-controls", ["/baseline-build/entry-controls"],
        extra=("--ro-bind", str(compiled), "/baseline-build", "--remount-ro", "/work"))
    run("ui-baseline-entry-compile", [*flags, "--target=" + ENV.WINDOWS_TARGET,
        "-ffreestanding", "-fno-lto", "-fstack-protector-strong", "-funwind-tables", "-Xclang", "-cfguard",
        "-c", "/work/fixture/" + UI_ENTRY_SOURCE, "-o", "/baseline-build/entry.obj"], extra=output_mounts)
    entry: Path = compiled / "entry.obj"
    if entry.is_symlink() or not entry.is_file() or not 0 < entry.stat().st_size <= 64 * 1024:
        raise ENV.Incomplete("Missing or oversized Windows UI baseline object.")
    admit_ui_entry_tools()
    return entry, sha256(entry)


def read_ui_entry(work: Path, source: Path, digest: str, image_digest: str) -> dict[str, object]:
    """Reconcile the guarded GUI PE with the final link's selected map rows."""
    map_path: Path = work / Path(UI_ENTRY_MAP).relative_to("/work")
    image: Path = work / "target" / ENV.WINDOWS_TARGET / "release" / (PACKAGE + ".exe")
    if map_path.is_symlink() or not map_path.is_file() or not 0 < map_path.stat().st_size <= UI_ENTRY_MAP_MAX_BYTES:
        raise ENV.Incomplete("Missing or oversized Windows UI baseline link map.")
    map_bytes: bytes = map_path.read_bytes()
    image_bytes: bytes = image.read_bytes()
    if hashlib.sha256(image_bytes).hexdigest() != image_digest:
        raise ENV.Incomplete("Windows UI bytes changed after version resource inspection.")
    pe: ModuleType = windows_pe_reader(source)
    try:
        observed: dict[str, object] = pe.ui_preentry_declarations(
            image_bytes, map_bytes.decode("utf-8"), object_sha256=digest)
    except ValueError as error:
        raise ENV.Incomplete(str(error)) from error
    return {**observed, "object_sha256": digest, "image_sha256": image_digest,
            "map_sha256": hashlib.sha256(map_bytes).hexdigest(),
            "compiler_inputs": {str(path): pin for path, pin in UI_ENTRY_TOOL_PINS.items()}}


def execute(root: Path, args, report: dict) -> None:
    """Freeze candidate inputs, run the selected row and reject input drift."""
    profile: str = getattr(args, "profile", "default")
    require_profile_lane(args.lane, profile)
    policy, source_identity = admit_product(root)
    artwork_dir = getattr(args, "msix_branding_dir", None)
    export_authoring = getattr(args, "export_msix_authoring", False)
    if artwork_dir is not None and (args.lane != "windows" or policy["schema_version"] != 3
                                    or getattr(args, "output_dir", None) is None):
        raise ENV.Incomplete("MSIX base artwork requires a Windows cross-build artifact output.")
    if export_authoring and (args.lane != "windows" or policy["schema_version"] != 3
                             or getattr(args, "output_dir", None) is None):
        raise ENV.Incomplete("MSIX authoring export requires a Windows cross-build artifact output.")
    if artwork_dir is not None and args.output_dir.resolve().is_relative_to(artwork_dir.absolute()):
        raise ENV.Incomplete("Windows artifact output overlaps the selected MSIX artwork.")
    base_artwork = read_msix_artwork(artwork_dir) if artwork_dir is not None else None
    if profile != "default":
        if policy["schema_version"] != 3:
            raise ENV.Incomplete("The selected profile requires its admitted schema-3 feature graph.")
        try:
            SLINT.profile_rows(policy, profile)
        except SLINT.Incomplete as error:
            raise ENV.Incomplete(str(error)) from error
    report["dependency_profile"] = profile
    sdk_directory = getattr(args, "linux_sdk_dir", None)
    if sdk_directory is not None and (args.lane != "portable" or policy["schema_version"] != 3):
        raise ENV.Incomplete("The optional Linux SDK belongs only to portable Slint checks.")
    rc_compiler = getattr(args, "llvm_rc", None)
    windows_ui_selected = args.lane == "windows" and policy["schema_version"] == 3
    ui_entry_requested: bool = getattr(args, "ui_baseline_entry", False)
    if ui_entry_requested:
        if not windows_ui_selected or getattr(args, "output_dir", None) is None:
            raise ENV.Incomplete("UI baseline entry requires the selected Windows artifact export.")
        admit_ui_entry_tools()
        source_identity.update({name: sha256(root / name) for name in (UI_ENTRY_SOURCE, UI_ENTRY_CONTROL_SOURCE)})
    if rc_compiler is not None and not windows_ui_selected:
        raise ENV.Incomplete("The LLVM resource compiler belongs only to the selected Windows build lane.")
    vendor = args.vendor_dir.resolve(strict=True)
    vendor_identity = admit_vendor(vendor, policy)
    toolchain = args.toolchain_dir.resolve(strict=True)
    ui_tools = admit_windows_ui_tools(toolchain, rc_compiler) if windows_ui_selected else None
    if args.lane == "windows":
        source_identity.update({name: sha256(root / name) for name in WINDOWS_PE_INPUT_FILES})
    if ui_tools:
        source_identity.update({name: sha256(root / name) for name in WINDOWS_UI_INPUT_FILES})
        for role in ("ui", "observe"):
            report[f"{role}_versioninfo"] = {
                "source_stage": "source", "tools": {mount: {"path": str(path), "sha256": digest}
                                                     for mount, (path, digest) in ui_tools.items()}}
    selected = args.deny if args.lane == "dependencies" else args.nextest if args.lane == "portable" else None
    extension = selected.resolve(strict=True) if selected else None
    report["tools"] = admit_tools(root, toolchain, extension, args.lane)
    report["source_sha256"] = source_identity
    report["lanes"] = []
    capture_requested = require_slint_capture(args, policy)
    if capture_requested:
        capture_output = args.output_dir.resolve()
        if (capture_output == root or capture_output.is_relative_to(root)
                or args.output_dir.exists() or args.output_dir.is_symlink()):
            raise ENV.Incomplete("Slint capture output must be new and outside the checkout.")
    sdk_inputs = None
    if sdk_directory is not None:
        sdk_directory = sdk_directory.resolve(strict=True)
        try:
            sdk_inputs = SDK.admit(sdk_directory, root / SDK.POLICY)
        except (SDK.Incomplete, OSError, ValueError) as error:
            raise ENV.Incomplete(str(error)) from error
        report["linux_sdk"] = sdk_inputs
        source_identity[SDK.POLICY] = sha256(root / SDK.POLICY)
    scratch = args.scratch_root.resolve(strict=True)
    if scratch == root or scratch.is_relative_to(root):
        raise ENV.Incomplete("Product scratch must be outside the checkout.")
    advisory = admit_advisories(args.advisory_receipt, time.time(), source_identity["Cargo.lock"]) if args.lane == "dependencies" else None
    if advisory:
        report["advisory_receipt_sha256"] = ADVISORY.receipt_digest(args.advisory_receipt)
    with ExitStack() as stack:
        temporary = Path(stack.enter_context(tempfile.TemporaryDirectory(prefix="nevrotick-product-", dir=scratch)))
        source, work = temporary / "source", temporary / "work"
        source.mkdir()
        work.mkdir()
        if advisory:
            (work / "cargo-home").mkdir()
            ADVISORY.export(advisory[0], work / "cargo-home", advisory[1])
        else:
            (work / "cargo-home").mkdir()
        (work / "cargo-home/config.toml").write_text(
            '[source.crates-io]\nreplace-with = "reviewed"\n[source.reviewed]\ndirectory = "/vendor"\n')
        (work / "fixture").mkdir()
        for name in source_identity:
            destination = source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(root / name, destination)
        if {name: sha256(source / name) for name in source_identity} != source_identity:
            raise ENV.Incomplete("Product inputs changed during snapshotting.")
        deny_base = (source / "deny.toml").read_text()
        if args.lane == "portable":
            config = source / ".config/nextest.toml"
            config.write_text(product_nextest(config.read_text()))
        library_inputs = None
        libraries = None
        staged_tools: dict[str, tuple[Path, str]] = {}
        linker_mounts: tuple[str, ...] = ()
        if args.lane == "windows":
            if args.xwin_inventory is None:
                raise ENV.Incomplete("Windows linking requires the admitted complete library inventory.")
            report["xwin_config_sha256"] = sha256(args.xwin_config)
            report["xwin_inventory_sha256"] = sha256(args.xwin_inventory)
            # Cargo scratch may be on DrvFS, which need not enforce private POSIX modes.
            tool_scratch = Path(stack.enter_context(tempfile.TemporaryDirectory(prefix="nevrotick-windows-tools-")))
            library_inputs = prepare_windows_link(root, toolchain, args.xwin_config, tool_scratch, args.xwin_inventory)
        if library_inputs:
            libraries, library_inventory_inputs, linker_input = library_inputs
            linker_mount, linker_path, linker_pin = linker_input
            report["windows_library_sha256"] = library_inventory_inputs
            linker_wrapper = tool_scratch / "linker"
            wrapper_pin = sha256(linker_wrapper)
            report["windows_linker"] = {"path": str(linker_path), "sha256": linker_pin, "wrapper_sha256": wrapper_pin}
            tool_inputs = {**(ui_tools or {}), linker_mount: (linker_path, linker_pin)}
            staged_tools = stage_windows_build_tools(tool_scratch / "build-tools", tool_inputs)
            linker_mounts = ("--ro-bind", str(linker_wrapper), "/work/linker",
                             *windows_tool_mounts(staged_tools, linker_mount))
            if ui_tools:
                linker_mounts += windows_tool_mounts(staged_tools, LINKER_LLVM,
                                                     f"/toolchain/{READOBJ_LIBRARY}")
        if args.lane == "dependencies":
            report["bootstrap_deny_sha256"] = sha256(source / "deny.toml")
        else:
            report["effective_deny_sha256"] = sha256(source / "deny.toml")
        report["effective_nextest_sha256"] = sha256(source / ".config/nextest.toml")
        checker_names = (*CHECKER_FILES, *UI_TEXT_CHECKER_FILES, *UI_NATIVE_INPUT_CHECKER_FILES) if policy["schema_version"] == 3 else CHECKER_FILES
        report["checker_sha256"] = {name: source_identity[name] for name in checker_names}
        metadata_outputs = {}
        tree_outputs = {}
        ep1_archive = work / ENGINE_EP1_ARCHIVE_NAME

        def run(label: str, command: list[str], *, extra: tuple[str, ...] = (),
                shared_deadline: float | None = None,
                discovered: set[CaseIdentity] | None = None) -> str | set[CaseIdentity]:
            target: str | None = None
            if label.startswith("dependency-"):
                flag = "--filter-platform" if label.startswith("dependency-metadata-") else "--target"
                target = command[command.index(flag) + 1]
                (source / "deny.toml").write_text(product_deny(deny_base, policy, target, profile=profile))
                report.setdefault("effective_deny_sha256_by_target", {})[target] = sha256(source / "deny.toml")
            clear_test_report(label, work)
            output_budget = ({"max_output": SLINT.MAX_METADATA_BYTES}
                             if policy["schema_version"] == 3 and label.startswith("dependency-metadata-") else {})
            if label == MSIX_INSPECTION_LANE:
                output_budget = {"max_output": MSIX_INSPECTION_MAX_OUTPUT}
            resources = command_resources(args.lane, label, command, profile=profile)
            invocation = sandbox(toolchain, work, source, vendor, command, extension, libraries,
                                 "cargo-deny" if args.lane == "dependencies" else "cargo-nextest", sdk_directory,
                                 lane=args.lane, label=label, profile=profile)
            if label in {"windows-link", "windows-engine-link"}:
                extra = (*linker_mounts, *extra)
            if extra:
                insertion = invocation.index("--")
                invocation[insertion:insertion] = extra
            timeout = resources["wall_seconds"]
            if shared_deadline is not None:
                deadline = shared_deadline

                def shared_wall_guard() -> str | None:
                    return "ENGINE_EP1_RELEASE_WALL" if time.monotonic() >= deadline else None

                timeout = max(0.0, deadline - time.monotonic())
                output_budget["resource_guard"] = shared_wall_guard
            if timeout == 0:
                result = {"command": invocation, "exit_code": None, "output": "",
                          "failure": "shared Engine EP1 release wall budget expired before launch"}
            else:
                result = ENV.run(invocation, cwd=root, timeout=timeout,
                                 environment={"PATH": "/usr/bin:/bin", "NEVROTICK_SYNTHETIC_CREDENTIAL": "synthetic-control"},
                                 **output_budget)
                if shared_deadline is not None and result["failure"] is None and shared_wall_guard():
                    result["failure"] = "shared Engine EP1 release wall budget expired"
                if result["failure"] is None and result["exit_code"] == 0 and label == ENGINE_EP1_RELEASE_BUILD_ROW:
                    try:
                        archive = ep1_archive.lstat()
                    except FileNotFoundError:
                        result["failure"] = "Engine EP1 release build produced no archive"
                    else:
                        if not stat.S_ISREG(archive.st_mode) or archive.st_nlink != 1 or archive.st_size == 0:
                            result["failure"] = "Engine EP1 release build produced an inadmissible archive"
            record = {"lane": label, "command": command, "exit_code": result["exit_code"],
                      "failure": result["failure"], "output": result["output"], "resource_profile": resources}
            if shared_deadline is not None:
                record["wall_timeout_seconds"] = timeout
            if label == MSIX_INSPECTION_LANE:
                # The final receipt contains the checked structured projection.
                del record["output"]
            if extra:
                record["sandbox_command"] = invocation
            report["lanes"].append(record)
            if label == MSIX_INSPECTION_LANE and (result["failure"] or result["exit_code"] != 0):
                raise ENV.Incomplete(f"Windows MSIX source inspection failed: "
                                     f"{result['failure'] or result['exit_code']}.")
            output: str = ENV.require_ok(result)
            if label in {"ui-text-controls", "ui-text-codegen-controls", "ui-text-native-controls",
                         "ui-native-input-controls", "ui-versioninfo-controls", "windows-pe-controls"}:
                record["executed_controls"] = validate_text_control_result(output)
            if label == "ui-text-coverage":
                record["text_coverage"] = validate_text_coverage_result(output)
            if label.startswith("dependency-metadata-"):
                if target is None:
                    raise ENV.Incomplete("Dependency metadata lacks its selected target.")
                record["dependency_metadata"] = validate_dependency_metadata(target, output, policy, profile=profile)
                if policy["schema_version"] == 3:
                    metadata_outputs[target] = output
                    tree_outputs[target] = {}
            if label.startswith("dependency-tree-"):
                if target not in metadata_outputs:
                    raise ENV.Incomplete("Selected tree needs its preceding validated metadata row.")
                try:
                    scope = (command[command.index("--package") + 1].partition("@")[0]
                             if "--package" in command else "workspace")
                    record["dependency_tree"] = SLINT.validate_tree(target, output, policy, scope=scope, profile=profile)
                    tree_outputs[target][scope] = output
                    scopes = SLINT.package_scopes(policy)
                    complete = set(tree_outputs[target]) == {"workspace", *scopes}
                    projection = (SLINT.selected_metadata(
                        target, metadata_outputs[target], tree_outputs[target]["workspace"], policy,
                        package_trees={package: tree_outputs[target][package] for package in scopes}, profile=profile)
                        if complete else None)
                except SLINT.Incomplete as error:
                    raise ENV.Incomplete(str(error)) from error
                if projection is not None:
                    projected = work / ("selected-metadata-" + target + ".json")
                    projected.write_text(json.dumps(projection, sort_keys=True, separators=(",", ":")))
                    record["selected_metadata_sha256"] = sha256(projected)
            validated: str | set[CaseIdentity] = output
            try:
                if label.startswith("test-discovery:"):
                    test_lane: str = label.removeprefix("test-discovery:")
                    record["planned_cases"] = sorted(planned_test_cases(test_lane, profile))
                    cases: set[CaseIdentity] = validate_test_discovery(test_lane, output, profile)
                    if not isinstance(cases, set):
                        raise ENV.Incomplete("Nextest discovery did not return a qualified case set.")
                    record["discovered_cases"] = sorted(cases)
                    validated = cases
                else:
                    counts = validate_test_result(label, output, work, profile=profile)
                    if counts:
                        if counts.get("planned_cases"):
                            if discovered is None or set(counts["executed_cases"]) != discovered:
                                raise ENV.Incomplete("Nextest execution identities differ from their discovery inventory.")
                            counts["discovered_cases"] = sorted(discovered)
                        record["tests"] = counts
            except ENV.Incomplete as error:
                record["failure"] = str(error)
                raise
            print(f"PASSED: {label}", flush=True)
            return validated

        def discover(label: str, command: list[str], *, deadline: float | None = None) -> set[CaseIdentity]:
            selection: str | set[CaseIdentity] = run("test-discovery:" + label, command, shared_deadline=deadline)
            if isinstance(selection, str):
                raise ENV.Incomplete("Nextest discovery command returned text instead of its qualified inventory.")
            return selection

        boundary_control(run)
        for label, command in native_input_control_commands(args.lane, schema_version=policy["schema_version"]):
            run(label, command)
        for label, command in text_contract_commands(args.lane, schema_version=policy["schema_version"]):
            run(label, command)
        if ui_tools:
            run("ui-versioninfo-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                            "discover", "-s", "scripts/tests", "-p", "test_windows_versioninfo.py", "-v"])
        if args.lane == "windows":
            run("windows-pe-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                        "discover", "-s", "scripts/tests", "-p", "test_windows_pe.py", "-v"])
        if ui_entry_requested:
            entry, entry_digest = compile_ui_entry(work, run)
        patched_names = tuple(record["name"] for record in policy.get("patched_packages", []))
        metadata_dir = None
        version_roles = {"windows-link": "ui", "windows-engine-link": "observe"}
        for label, command in commands(args.lane, schema_version=policy["schema_version"], patched_packages=patched_names, profile=profile):
            if label == ENGINE_EP1_RELEASE_BUILD_ROW:
                raise ENV.Incomplete("The Engine EP1 release archive row belongs to optimized test dispatch.")
            if args.lane == "portable" and profile == SLINT.EP1_PROFILE and label == ENGINE_EP1_RELEASE_TEST_ROW:
                if tuple(command) != ENGINE_EP1_RELEASE_TEST:
                    raise ENV.Incomplete("The configured Engine EP1 optimized command differs from its archive selectors.")
                extract = work / ENGINE_EP1_EXTRACT_NAME
                if os.path.lexists(ep1_archive) or os.path.lexists(extract):
                    raise ENV.Incomplete("Engine EP1 release archive and extraction paths must be fresh.")
                reuse = list(ENGINE_EP1_RELEASE_REUSE)
                resources = command_resources(args.lane, label, reuse, profile=profile)
                deadline = time.monotonic() + resources["wall_seconds"]
                run(ENGINE_EP1_RELEASE_BUILD_ROW, list(ENGINE_EP1_RELEASE_BUILD), shared_deadline=deadline)
                extract.mkdir(mode=0o700)
                discovered: set[CaseIdentity] = discover(label, list(ENGINE_EP1_RELEASE_DISCOVERY), deadline=deadline)
                run(label, reuse, shared_deadline=deadline, discovered=discovered)
                continue
            if planned_test_cases(label, profile):
                selection: set[CaseIdentity] = discover(label, nextest_discovery_command(command))
                run(label, command, discovered=selection)
                continue
            if not ui_tools or label not in version_roles:
                run(label, command)
                continue
            if metadata_dir is None:
                # The metadata producer requires an owner-private POSIX parent.
                # The Cargo scratch may be a separately admitted DrvFS location.
                metadata_dir = Path(stack.enter_context(tempfile.TemporaryDirectory(prefix="nevrotick-versioninfo-", dir="/tmp")))
                generate_windows_metadata(metadata_dir / "generation", run)
                (work / "metadata").mkdir()
            role = version_roles[label]
            resource, expected, inputs = compile_windows_versioninfo(metadata_dir, role, staged_tools, run)
            receipt = report[f"{role}_versioninfo"]
            receipt.update(inputs)
            digest = inputs["res_sha256"]
            if role == "ui":
                extra = ("--ro-bind", str(resource), RESOURCE_PATH, "--setenv", RESOURCE_DIGEST_ENV, digest)
                if ui_entry_requested:
                    extra += ("--ro-bind", str(entry), ui_entry_object_path(entry_digest))
                    command = link_ui_entry(command, entry_digest)
            else:
                extra = ("--ro-bind", str(resource), observe_resource_path(digest))
                command = link_observe_resource(command, digest)
            run(label, command, extra=extra)
            if sha256(resource) != digest:
                raise ENV.Incomplete(f"Selected {role} resource changed during linking.")
            receipt.update(read_windows_versioninfo(work, source, role, expected, staged_tools, run))
            if role == "ui" and ui_entry_requested:
                if sha256(entry) != entry_digest:
                    raise ENV.Incomplete("Windows UI baseline object changed during linking.")
                report["ui_preentry"] = read_ui_entry(work, source, entry_digest, receipt["image_sha256"])
        if capture_requested:
            run("slint-capture-build", slint_capture_build_command())
            run("slint-capture", ["/work/target/debug/examples/capture"])
        if args.lane == "portable":
            report["test_counts"] = summarize_test_counts(
                report["lanes"], patched_packages=patched_names, schema_version=policy["schema_version"])
        msix_inspection = None
        msix_authoring = None
        if ui_tools:
            generation = sha256(metadata_dir / "generation/tree/receipt.json")
            msix_inspection = inspect_windows_msix(metadata_dir, generation, run, source=source,
                                                   linked_ui=report["ui_versioninfo"],
                                                   require_base_artwork=base_artwork is not None)
            build = work / "target" / ENV.WINDOWS_TARGET / "release"
            for role, filename in (("ui", f"{PACKAGE}.exe"), ("observe", "nevrotick-observe.exe")):
                if report[f"{role}_versioninfo"].get("image_sha256") != sha256(build / filename):
                    raise ENV.Incomplete(f"Windows {role} resource inspection or linked image custody is incomplete.")
            if (any(report[f"{role}_versioninfo"]["generation_sha256"] != generation for role in version_roles.values())
                    or sha256(metadata_dir / "generation/tree/receipt.json") != generation
                    or admit_windows_ui_tools(toolchain, rc_compiler) != ui_tools):
                raise ENV.Incomplete("Windows version resource source or tool custody is incomplete.")
            if export_authoring:
                msix_authoring = read_msix_authoring_snapshot(metadata_dir / "generation/tree", msix_inspection)
        final_source = admit_product(root)[1]
        if ui_entry_requested:
            final_source.update({name: sha256(root / name) for name in (UI_ENTRY_SOURCE, UI_ENTRY_CONTROL_SOURCE)})
            admit_ui_entry_tools()
        if args.lane == "windows":
            final_source.update({name: sha256(root / name) for name in WINDOWS_PE_INPUT_FILES})
        if ui_tools:
            final_source.update({name: sha256(root / name) for name in WINDOWS_UI_INPUT_FILES})
        if sdk_inputs:
            final_source[SDK.POLICY] = sha256(root / SDK.POLICY)
            try:
                if SDK.admit(sdk_directory, root / SDK.POLICY) != sdk_inputs:
                    raise ENV.Incomplete("Native SDK inputs changed during verification.")
            except (SDK.Incomplete, OSError, ValueError) as error:
                raise ENV.Incomplete(str(error)) from error
        if final_source != source_identity or admit_vendor(vendor, policy) != vendor_identity:
            raise ENV.Incomplete("Source or dependency inputs changed during product verification.")
        if library_inputs and (library_inventory(libraries) != library_inputs[1]
                               or sha256(args.xwin_inventory) != report["xwin_inventory_sha256"]
                               or sha256(args.xwin_config) != report["xwin_config_sha256"]):
            raise ENV.Incomplete("Microsoft library inputs changed during product verification.")
        if library_inputs:
            _, linker_path, linker_pin = library_inputs[2]
            ENV.require_admitted_binary(linker_path, linker_pin, "rust-lld")
            if sha256(linker_wrapper) != wrapper_pin:
                raise ENV.Incomplete("Windows linker wrapper digest changed before export.")
            for path, pin in sorted(set(staged_tools.values())):
                if sha256(path) != pin:
                    raise ENV.Incomplete(f"Staged Windows tool digest changed before export: {path.name}")
        if advisory:
            if (admit_advisories(args.advisory_receipt, time.time(), source_identity["Cargo.lock"]) != advisory
                    or ADVISORY.receipt_digest(args.advisory_receipt) != report["advisory_receipt_sha256"]):
                raise ENV.Incomplete("Advisory/index snapshot changed or expired during verification.")
            report["advisory_freshness"] = "separate acquisition receipt and snapshot verified; at most 24 hours old"
        if artwork_dir is not None and read_msix_artwork(artwork_dir) != base_artwork:
            raise ENV.Incomplete("Windows MSIX base artwork changed during verification.")
        if base_artwork is not None and msix_inspection is None:
            raise ENV.Incomplete("Windows MSIX base artwork has no checked authoring generation.")
        if msix_inspection is not None:
            report["msix_assembly_inputs"] = msix_inspection
        if args.output_dir:
            output = (args.output_dir.parent.resolve(strict=True) / args.output_dir.name
                      if args.lane == "windows" else args.output_dir.resolve())
            if output == root or output.is_relative_to(root):
                raise ENV.Incomplete("Artifact output must be outside the checkout.")
            if artwork_dir is not None and output.is_relative_to(artwork_dir.absolute()):
                raise ENV.Incomplete("Windows artifact output overlaps the selected MSIX artwork.")
            if args.lane == "windows":
                export_windows_build(work, output, source=source, report=report,
                                     base_artwork=base_artwork, authoring=msix_authoring)
            else:
                output.mkdir(mode=0o700)
                if args.lane == "portable":
                    report["engine_artifact"] = export_host_engine_artifact(work, output)
                    if capture_requested:
                        report["slint_captures"] = export_slint_captures(work, output)
                report["status"] = "passed"
                (output / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
        else:
            report["status"] = "passed"


def verification_receipt_destination(args, root: Path) -> Path:
    """Keep a requested JSON receipt away from protected inputs and export slots."""
    try:
        receipt = ENV.external_receipt(args.json, root)
        protected = [directory.resolve() for directory in (args.output_dir, args.msix_branding_dir)
                     if directory is not None]
    except ENV.Incomplete:
        raise
    except RuntimeError as error:
        raise ENV.Incomplete("Verification receipt or output path could not be resolved.") from error
    if any(receipt.is_relative_to(directory) for directory in protected):
        raise ENV.Incomplete("Verification receipt overlaps protected MSIX inputs or artifact output.")
    return receipt


def main(argv=None) -> int:
    """Resolve explicit/default paths without installing or fetching anything."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("portable", "windows", "dependencies"), default="portable")
    parser.add_argument("--profile", choices=("default", SLINT.EP1_PROFILE, *SLINT.RGT_PROFILES), default="default")
    parser.add_argument("--capture-slint", action="store_true")
    parser.add_argument("--inspect-msix-handoff", type=Path,
                        help="read a retained private Windows artifact export without building")
    parser.add_argument("--handoff-receipt-sha256",
                        help="independently retained SHA-256 for the export's verification.json")
    parser.add_argument("--export-msix-authoring", action="store_true",
                        help="retain private checked assembly source in a Windows artifact output")
    parser.add_argument("--ui-baseline-entry", action="store_true",
                        help="link the selected Windows UI through the baseline CPU/OS guard")
    build_paths = ("vendor-dir", "scratch-root", "toolchain-dir", "nextest", "deny", "xwin-config", "xwin-inventory",
                   "advisory-receipt", "output-dir", "json", "linux-sdk-dir", "llvm-rc", "msix-branding-dir")
    for name in build_paths:
        parser.add_argument("--" + name, type=Path)
    args = parser.parse_args(argv)
    if args.inspect_msix_handoff is not None or args.handoff_receipt_sha256 is not None:
        if (not args.handoff_receipt_sha256 or args.inspect_msix_handoff is None
                or args.capture_slint or args.export_msix_authoring or args.ui_baseline_entry
                or args.lane != "portable" or args.profile != "default"
                or any(getattr(args, name.replace("-", "_")) is not None for name in build_paths)):
            parser.error("MSIX handoff inspection requires both handoff options without build options.")
        try:
            handoff = inspect_msix_handoff(ROOT, args.inspect_msix_handoff, args.handoff_receipt_sha256)
        except ENV.Incomplete as error:
            print(f"INCOMPLETE: {error}", file=sys.stderr)
            return 1
        except (RuntimeError, OSError, ValueError, KeyError, TypeError, AttributeError):
            print("INCOMPLETE: Invalid or changed Windows MSIX handoff.", file=sys.stderr)
            return 1
        print(json.dumps(handoff, indent=2))
        return 0
    report = {"scope": f"UI-M0, wire, domain, exchange, synthetic regression and controlled-loopback engine pilot {args.lane} verification", "status": "incomplete",
              "native_windows_execution": "not performed", "advisory_freshness": "not checked by this row"}
    receipt = None
    try:
        if args.json is not None:
            receipt = verification_receipt_destination(args, ROOT)
        defaults = json.loads(POINTER.read_text()) if POINTER.is_file() else {}
        rust = json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]
        defaults.setdefault("toolchain_dir", str(Path.home() / ".rustup/toolchains" / (rust["channel"] + "-" + rust["host"])))
        defaults.setdefault("scratch_root", tempfile.gettempdir())
        defaults.setdefault("nextest", shutil.which("cargo-nextest"))
        defaults.setdefault("deny", shutil.which("cargo-deny"))
        defaults.setdefault("xwin_config", str(Path.home() / ".local/share/nevrotick/xwin.json"))
        for key in ("vendor_dir", "scratch_root", "toolchain_dir", "nextest", "deny", "xwin_config", "xwin_inventory", "advisory_receipt"):
            if getattr(args, key) is None and defaults.get(key):
                setattr(args, key, Path(defaults[key]))
        if args.vendor_dir is None:
            raise ENV.Incomplete("No reviewed vendor directory; run the documented separate acquisition first.")
        execute(ROOT, args, report)
        report["status"] = "passed"
    except WindowsOutputPublicationUncertain as error:
        report["status"], report["error"] = "publication-uncertain", str(error)
    except (ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration, ET.ParseError) as error:
        if report["status"] == "publication-uncertain":
            report["error"] += f" Verification cleanup failed ({type(error).__name__})."
        else:
            report["status"] = "incomplete"
            report["error"] = str(error)
            if args.lane == "windows":
                report["error"] = "\n".join((report["error"], *getattr(error, "__notes__", ())))
    if receipt is not None:
        try:
            if verification_receipt_destination(args, ROOT) != receipt:
                raise ENV.Incomplete("Verification receipt destination changed during execution.")
            receipt.write_text(json.dumps(report, indent=2) + "\n")
        except (ENV.Incomplete, OSError) as error:
            if report["status"] == "publication-uncertain":
                report["error"] += f" External JSON receipt failed ({type(error).__name__})."
            else:
                report["status"], report["error"] = "incomplete", str(error)
    print(f"{report['status'].upper()}: {report['scope']}")
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
