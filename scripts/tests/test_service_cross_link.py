"""Exercise service cross-link export with inert emitted PE/PDB fixtures."""
from __future__ import annotations

import contextlib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import struct
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import MagicMock, patch
from uuid import UUID

from test_windows_pe import _image, _OBSERVE_GUID, _OPTIONAL, _pdb, _UI_GUID, _ui_resource_image, with_import_modules

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "service_cross_link_under_test", ROOT / "scripts/cross-link-service-fixture.py",
)
CROSS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CROSS)
ENGINE, BROKER = CROSS.BINS


class BaselineServiceEntryReadback(unittest.TestCase):
    MODULES: tuple[str, ...] = ("kernel32.dll", "vcruntime140.dll")
    MAP: str = (
        " Preferred load address is 0000000140000000\n\n"
        "  Address         Publics by Value              Rva+Base               Lib:Object\n\n"
        " 0001:00000010       nevrotick_service_entry      0000000140001010     service-entry.obj\n"
        " 0001:00000030       mainCRTStartup               0000000140001030     msvcrt:exe_main.obj\n"
        "\n entry point at         0001:00000010\n"
    )

    def check(self, map_text: str, modules: tuple[str, ...] = MODULES) -> dict[str, object]:
        image: bytearray = _ui_resource_image(b"inert version", b"inert manifest")
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        struct.pack_into("<H", image, _OPTIONAL + 68, 3)
        image = with_import_modules(image, direct=modules)
        with patch.object(CROSS, "SERVICE_PREENTRY_IMPORTS", self.MODULES):
            return CROSS.read_guarded_service_entry(bytes(image), map_text)

    def test_binds_distinct_gate_and_original_crt(self) -> None:
        self.assertEqual(self.check(self.MAP)["crt_startup_rva"], 0x1030)

    def test_refuses_the_wrong_entry_or_object(self) -> None:
        with self.assertRaisesRegex(CROSS.Incomplete, "nevrotick_service_entry segment and preferred address disagree"):
            self.check(self.MAP.replace("140001010", "140001040"))
        with self.assertRaisesRegex(CROSS.Incomplete, "gate object differs"):
            self.check(self.MAP.replace("service-entry.obj", "other.obj"))

    def test_refuses_a_duplicate_linked_symbol(self) -> None:
        row: str = next(line for line in self.MAP.splitlines() if "nevrotick_service_entry" in line)
        with self.assertRaisesRegex(CROSS.Incomplete, "nevrotick_service_entry.*ambiguous"):
            self.check(self.MAP.replace(row, row + "\n" + row))

    def test_requires_the_crt_row_to_match_executable_image_bytes(self) -> None:
        for address, reason in (
            ("140003000", "lies outside the image"),
            ("140002000", "has no executable file-backed section"),
            ("140001040", "segment and preferred address disagree"),
        ):
            with self.subTest(address=address), self.assertRaisesRegex(CROSS.Incomplete, reason):
                self.check(self.MAP.replace("140001030", address))

    def test_refuses_a_new_preentry_import(self) -> None:
        with self.assertRaisesRegex(CROSS.Incomplete, "loader import inventory changed"):
            self.check(self.MAP, (*self.MODULES, "additional.dll"))


class ServiceArtifactExport(unittest.TestCase):
    def setUp(self):
        scratch = tempfile.TemporaryDirectory()
        self.addCleanup(scratch.cleanup)
        private = Path(scratch.name)
        self.root = private / "source"
        self.records = self.root / "records"
        self.records.mkdir(parents=True)
        self.cache = private / "cache"
        self.scratch = private / "scratch"
        self.scratch.mkdir(mode=0o700)
        self.stable = private / "stable"
        self.vendor = private / "vendor"
        self.vendor.mkdir()
        self.libraries = private / "libraries"
        self.libraries.mkdir(mode=0o700)
        self.llvm = self.stable / "lib/rustlib/x86_64-unknown-linux-gnu/bin"
        self.llvm.mkdir(parents=True)
        for tool in ("rust-lld", "llvm-readobj"):
            path = self.llvm / tool
            path.touch()
            path.chmod(0o700)
        for name in (CROSS.SELF, CROSS.PE_READER, CROSS.METRICS_EXPORT_HELPER, ".cargo/config.toml"):
            destination = self.root / name
            destination.parent.mkdir(exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        self.index = {"path": "fixture-index", "sha256": "fixture-index-sha"}
        self.core_policy = {"collector_kind": "pipe", "limit_bytes": 1}
        self.identity = {CROSS.SELF: CROSS.C.PRODUCT.sha256(self.root / CROSS.SELF)}
        self.checkpoint = self.records / "source.json"
        self.sdk = private / "sdk.json"
        self.sdk_inventory = private / "sdk-inventory.json"
        for path in (self.checkpoint, self.sdk, self.sdk_inventory):
            path.write_text("{}")
        self.sdk.write_text(json.dumps({"root": str(self.libraries)}))
        self.tools = {
            "stable": self.stable,
            "inventories": {},
            "readonly_roots": [self.stable],
            "installation_pointer": "fixture-installation",
            "installation_pointer_sha256": "fixture-installation-sha",
            "policy_sha256": "fixture-policy-sha",
        }
        self.pairs = {
            ENGINE: (_image(3, pdb_path=ENGINE.replace("-", "_") + ".pdb", guid=_UI_GUID), _pdb(guid=_UI_GUID)),
            BROKER: (_image(3, pdb_path=BROKER + ".pdb", guid=_OBSERVE_GUID), _pdb(guid=_OBSERVE_GUID)),
        }
        self.symbols = {ENGINE: ENGINE.replace("-", "_") + ".pdb", BROKER: BROKER + ".pdb"}
        self.role_bindings: dict[str, CROSS.C.PRODUCT.STRICT.WindowsReleaseRole] = {
            "lab-" + role: CROSS.C.PRODUCT.STRICT.WindowsReleaseRole(
                "lab-" + role, stem + ".exe", "nevrotick-windows-service-fixture",
                "tools/windows-service-fixture/Cargo.toml",
                "tools/windows-service-fixture/src/bin/" + role + ".rs", "declared-bin", (),
                "console", "fixture", "SCM-laboratory-controller", "rust-entry-source-only",
                CROSS.SELF + ":run", "pending-complete-role-qualification",
            ) for role, stem in (("engine", ENGINE), ("broker", BROKER))
        }
        # These tiny fixtures replace only the shared, checked source admission.
        # Its real manifest/Cargo/entry checks have their own receiving controls.
        self.role_admission: MagicMock = self.enterContext(patch.object(
            CROSS.C.PRODUCT.STRICT, "windows_lab_service_role_bindings", return_value=self.role_bindings,
        ))
        for object_, name, replacement in (
            (CROSS, "ROOT", self.root),
            (CROSS.Q, "ROOT", self.root),
            (CROSS.Q, "RECORDS", self.records),
            (CROSS.Q, "CACHE", self.cache),
            (CROSS.Q, "DISK_FLOOR", 0),
            (CROSS.Q, "MEMORY_FLOOR", 0),
        ):
            self.enterContext(patch.object(object_, name, replacement))
        checkpoint = {
            "candidate_sha256": "fixture-candidate",
            "index_after": self.index,
            "tools": {"stable": str(self.stable)},
            "vendor": {"path": str(self.vendor), "inventory_sha256": CROSS.Q.canonical({}), "files": 0},
        }
        for object_, name, output in (
            (CROSS.Q, "no_ambient_flags", None),
            (CROSS.Q, "disable_core_dumps", self.core_policy),
            (CROSS.Q, "check_core_dumps", self.core_policy),
            (CROSS, "source_prerequisites", ({}, self.identity, checkpoint, {},
                                             hashlib.sha256(self.checkpoint.read_bytes()).hexdigest())),
            (CROSS.Q, "source_identity", ({}, self.identity)),
            (CROSS.Q, "admit_service_tools", {}),
            (CROSS.Q, "tool_identity", checkpoint["tools"]),
            (CROSS.Q, "index_identity", self.index),
            (CROSS.C.ANALYSIS, "admit", self.tools),
            (CROSS.C.PRODUCT, "windows_libraries", (self.libraries, {})),
            (CROSS.C.PRODUCT, "library_inventory", {}),
            (CROSS.C.PRODUCT, "admit_vendor", {}),
        ):
            self.enterContext(patch.object(object_, name, return_value=output))
        self.runtime: MagicMock = self.enterContext(patch.object(
            CROSS.C.RUNTIME, "run", side_effect=self.execute_fixture,
        ))
        self.enterContext(contextlib.redirect_stdout(io.StringIO()))

    def execute_fixture(self, root, work, command, **_kwargs):
        release = Path("target") / CROSS.TARGET / "release"
        if command in ([str(self.llvm / "rust-lld"), "-flavor", "link", "--version"],
                       [str(self.llvm / "llvm-readobj"), "--version"]):
            return {"exit_code": 0, "failure": None, "output": "inert version response"}
        if command[0] == str(self.stable / "bin/cargo"):
            self.assertEqual(root, self.root)
            build = work / release
            build.mkdir(parents=True)
            ledger = [str(self.llvm / "rust-lld"), "-flavor", "link"]
            ledger.extend("/libpath:" + str(self.libraries / scope) for scope in CROSS.LIBRARY_SCOPES)
            calls = []
            output = []
            for name, (image, symbols) in self.pairs.items():
                leaf = name + ".exe"
                (build / leaf).write_bytes(image)
                (build / self.symbols[name]).write_bytes(symbols)
                calls.append(json.dumps([*ledger, "/out:" + leaf]))
                output.append(json.dumps({
                    "reason": "compiler-artifact",
                    "executable": str(Path("/work") / release / leaf),
                    "target": {"name": name, "kind": ["bin"]},
                    "profile": {"opt_level": "3", "debug_assertions": False},
                }))
            (work / "linker-calls.jsonl").write_text("\n".join(calls) + "\n")
            return {"exit_code": 0, "failure": None, "output": "\n".join(output)}
        self.assertEqual(command[:5], [
            str(self.llvm / "llvm-readobj"), "--file-headers", "--coff-load-config",
            "--coff-debug-directory", "--coff-imports",
        ])
        self.assertIn(command[5], [str(Path("/work") / release / (name + ".exe")) for name in CROSS.BINS])
        return {"exit_code": 0, "failure": None, "output": "inspected " + Path(command[5]).name}

    def run_link(self, label, *, scratch_root=None, baseline_entry: bool = False):
        return CROSS.run(SimpleNamespace(
            label=label, lane="link", source_checkpoint=self.checkpoint,
            sdk_config=self.sdk, sdk_inventory=self.sdk_inventory, vendor_dir=self.vendor,
            scratch_root=scratch_root, baseline_entry=baseline_entry,
        ))

    def receipt(self, label):
        return json.loads((self.records / ("cross-" + label + ".json")).read_text())

    def assert_healthy(self, label):
        self.assertEqual(self.run_link(label), "passed")
        receipt = self.receipt(label)
        self.assertTrue(receipt["bookends_passed"])
        self.assertTrue(receipt["index_bytes_unchanged"])
        self.assertEqual(receipt["index_before"], receipt["index_after"])
        self.assertEqual(receipt["index_after"], self.index)
        self.assertEqual(receipt["source_sha256"], self.identity)
        self.assertEqual(receipt["checkpoint_sha256"], hashlib.sha256(b"{}").hexdigest())
        self.assertFalse(receipt["native_execution"])
        self.assertEqual(receipt["limits"]["core_dump_policy"], self.core_policy)
        self.assertEqual(receipt["limits"]["core_bytes"], 1)
        self.assertEqual(len(receipt["commands"]), 3)
        self.assertEqual(set(receipt["artifacts"]), {ENGINE, BROKER})
        directory = self.records / ("cross-" + label + "-artifacts")
        expected_names = set()
        for name, (image, symbols) in self.pairs.items():
            artifact = receipt["artifacts"][name]
            image_name = name + ".exe"
            symbol_name = self.symbols[name]
            log_name = name + "-llvm-readobj.txt"
            expected_names.update((image_name, symbol_name, log_name))
            self.assertEqual(self.root / artifact["image"], directory / image_name)
            self.assertEqual(self.root / artifact["pdb"], directory / symbol_name)
            self.assertEqual((directory / image_name).read_bytes(), image)
            self.assertEqual((directory / symbol_name).read_bytes(), symbols)
            self.assertEqual((directory / log_name).read_text(), "inspected " + image_name)
            self.assertEqual(artifact["image_sha256"], hashlib.sha256(image).hexdigest())
            self.assertEqual(artifact["pdb_sha256"], hashlib.sha256(symbols).hexdigest())
        self.assertEqual({path.name for path in directory.iterdir()}, expected_names)

    def assert_rejected_then_restored(self, label, bad_symbols):
        image, expected_symbols = self.pairs[BROKER]
        self.pairs[BROKER] = image, bad_symbols
        with self.assertRaisesRegex(CROSS.Incomplete, "PDB GUID or age"):
            self.run_link(label)
        self.assertFalse((self.records / ("cross-" + label + "-artifacts")).exists())
        receipt = self.receipt(label)
        self.assertEqual(receipt["status"], "incomplete")
        self.assertFalse(receipt["bookends_passed"])
        self.assertFalse(receipt["native_execution"])
        self.assertNotIn("artifacts", receipt)
        self.assertIn("PDB GUID or age", receipt["custody_error"])
        self.assertEqual({path.name for path in self.records.iterdir()}, {"source.json", "cross-" + label + ".json"})
        self.pairs[BROKER] = image, expected_symbols
        restored = label + "-restored"
        self.assert_healthy(restored)
        self.assertEqual(self.receipt(restored)["artifacts"][BROKER]["pdb_identity"], {
            "guid": str(UUID(bytes_le=_OBSERVE_GUID)),
            "age": 1,
            "sha256": hashlib.sha256(expected_symbols).hexdigest(),
        })

    def test_valid_fixed_roles_export_exact_bytes_and_receipt(self):
        self.assert_healthy("healthy")

    def test_fixed_service_exports_record_their_checked_source_roles(self) -> None:
        self.assert_healthy("source-roles")
        receipt: dict[str, object] = self.receipt("source-roles")
        stem: str
        role: str
        for stem, role in ((ENGINE, "lab-engine"), (BROKER, "lab-broker")):
            artifact: dict[str, object] = receipt["artifacts"][stem]
            self.assertIn("source_role", artifact)
            self.assertEqual(artifact["source_role"]["role"], role)
            self.assertEqual(artifact["source_role"]["artifact"], stem + ".exe")
            self.assertEqual(artifact["source_role"]["entry_scope"], "rust-entry-source-only")
            self.assertEqual(artifact["source_role"]["runtime_policy"], "pending-complete-role-qualification")
            self.assertEqual(artifact["selected_preentry_owner"], "toolchain")
        self.assertEqual(self.role_admission.call_args_list[0].args, (self.root,))
        self.assertEqual(self.role_admission.call_args_list[1].args,
                         (self.cache / "cross-source-roles/source",))

    def test_wrong_fixed_binding_refuses_ready_and_link_before_work_or_commands(self) -> None:
        changed: dict[str, CROSS.C.PRODUCT.STRICT.WindowsReleaseRole] = dict(self.role_bindings)
        changed["lab-engine"] = changed["lab-engine"]._replace(artifact="other.exe")
        self.role_admission.return_value = changed
        lane: str
        for lane in ("ready", "link"):
            with self.subTest(lane=lane), self.assertRaisesRegex(CROSS.Incomplete, "selected cross-link target"):
                CROSS.run(SimpleNamespace(label="wrong-" + lane, lane=lane, scratch_root=None))
            self.assertFalse(self.cache.exists())
            self.runtime.assert_not_called()
            self.assertEqual({path.name for path in self.records.iterdir()}, {"source.json"})
        self.role_admission.return_value = self.role_bindings
        self.assert_healthy("restored-role-binding")

    def test_copied_source_binding_drift_refuses_before_compiler_dispatch(self) -> None:
        changed: dict[str, CROSS.C.PRODUCT.STRICT.WindowsReleaseRole] = dict(self.role_bindings)
        changed["lab-engine"] = changed["lab-engine"]._replace(runtime_policy="different-policy")
        self.role_admission.side_effect = [self.role_bindings, changed]
        with self.assertRaisesRegex(CROSS.Incomplete, "Copied laboratory service role bindings differ"):
            self.run_link("copied-role-drift")
        self.runtime.assert_not_called()
        self.assertEqual(self.receipt("copied-role-drift")["commands"], [])
        self.assertFalse((self.records / "cross-copied-role-drift-artifacts").exists())

    def test_ready_checks_original_and_copied_bindings(self) -> None:
        status: str = CROSS.run(SimpleNamespace(
            label="ready-roles", lane="ready", source_checkpoint=self.checkpoint,
            sdk_config=self.sdk, sdk_inventory=self.sdk_inventory, vendor_dir=self.vendor,
            scratch_root=None,
        ))
        self.assertEqual(status, "passed")
        self.assertEqual(self.role_admission.call_count, 2)
        self.assertEqual(self.runtime.call_count, 2)
        self.assertNotIn("artifacts", self.receipt("ready-roles"))

    def test_guarded_exports_keep_rust_scope_and_selected_recipe_entry(self) -> None:
        entry: Path = self.root / CROSS.SERVICE_ENTRY_SOURCE
        entry.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / CROSS.SERVICE_ENTRY_SOURCE, entry)
        self.identity[CROSS.SERVICE_ENTRY_SOURCE] = CROSS.C.PRODUCT.sha256(entry)
        compiler: Path = self.stable / "clang"
        compiler.write_bytes(b"inert compiler identity")
        compiler.chmod(0o700)
        stem: str
        for stem in (ENGINE, BROKER):
            image: bytearray = bytearray(self.pairs[stem][0])
            struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
            struct.pack_into("<II", image, _OPTIONAL + 32, 0x1000, 0x200)
            struct.pack_into("<I", image, _OPTIONAL + 60, 0x200)
            self.pairs[stem] = (with_import_modules(image, direct=BaselineServiceEntryReadback.MODULES),
                                self.pairs[stem][1])

        def execute(root: Path, work: Path, command: list[str], **options: object) -> dict[str, object]:
            if command[0] == str(compiler):
                self.assertEqual(command[-4:], ["-c", "/work/source/" + CROSS.SERVICE_ENTRY_SOURCE,
                                               "-o", "/work/" + CROSS.SERVICE_ENTRY_OBJECT])
                (work / CROSS.SERVICE_ENTRY_OBJECT).write_bytes(b"inert baseline object")
                return {"exit_code": 0, "failure": None, "output": "inert object emission"}
            result: dict[str, object] = self.execute_fixture(root, work, command, **options)
            if command[0] == str(self.stable / "bin/cargo"):
                name: str
                for name in (ENGINE, BROKER):
                    (work / (name.replace("-", "_") + ".map")).write_text(BaselineServiceEntryReadback.MAP)
            return result

        with (patch.object(CROSS, "BASELINE_CLANG", compiler),
              patch.object(CROSS, "BASELINE_CLANG_INPUTS", {compiler: CROSS.C.PRODUCT.sha256(compiler)}),
              patch.object(CROSS, "SERVICE_PREENTRY_IMPORTS", BaselineServiceEntryReadback.MODULES),
              patch.object(CROSS.C.RUNTIME, "run", side_effect=execute)):
            self.assertEqual(self.run_link("guarded-source-roles", baseline_entry=True), "passed")
        receipt: dict[str, object] = self.receipt("guarded-source-roles")
        self.assertEqual(len(receipt["commands"]), 4)
        for stem in (ENGINE, BROKER):
            artifact: dict[str, object] = receipt["artifacts"][stem]
            self.assertEqual(artifact["selected_preentry_owner"], CROSS.SELF + ":run")
            self.assertEqual(artifact["source_role"]["entry_scope"], "rust-entry-source-only")
            self.assertEqual(artifact["raw_pe"]["baseline_entry"]["entry_rva"], 0x1010)
            self.assertEqual(artifact["raw_pe"]["baseline_entry"]["crt_startup_rva"], 0x1030)
        self.assertFalse(receipt["native_execution"])

    def test_late_empty_artifact_directory_is_preserved(self):
        label: str = "late-artifact-slot"
        artifacts: Path = self.records / ("cross-" + label + "-artifacts")
        is_symlink = Path.is_symlink
        prior: os.stat_result | None = None

        def appear_after_check(path: Path) -> bool:
            nonlocal prior
            result: bool = is_symlink(path)
            if path == artifacts and prior is None:
                path.mkdir()
                prior = path.stat()
            return result

        with (patch.object(Path, "is_symlink", appear_after_check),
              self.assertRaises(FileExistsError)):
            self.run_link(label)

        self.assertIsNotNone(prior)
        self.assertTrue(os.path.samestat(prior, artifacts.stat()))
        self.assertEqual(list(artifacts.iterdir()), [])
        receipt = self.receipt(label)
        self.assertEqual(receipt["status"], "incomplete")
        self.assertFalse(receipt["bookends_passed"])
        self.assertNotIn("artifacts", receipt)
        self.assertFalse(list(self.records.glob(".cross-publish-*")))
        self.assert_healthy("uncontended-artifact-slot")

    def test_reserve_boundary_and_existing_record_have_distinct_diagnostics(self):
        minimum = 4 * 1024 ** 3
        free = SimpleNamespace(free=minimum - 1)
        with patch.object(CROSS.Q, "DISK_FLOOR", minimum), patch.object(CROSS.shutil, "disk_usage", return_value=free):
            with self.assertRaisesRegex(CROSS.Incomplete, f"required {minimum} bytes, available {free.free} bytes"):
                self.run_link("disk-boundary")
            self.assertFalse(self.cache.exists())
            free.free = minimum
            self.assert_healthy("disk-boundary")
            receipt = self.records / "cross-disk-boundary.json"
            previous = receipt.read_bytes()
            with self.assertRaisesRegex(CROSS.Incomplete, "prior cross-link record already exists"):
                self.run_link("disk-boundary")
            self.assertEqual(receipt.read_bytes(), previous)

    def test_disk_reserve_lost_before_command_uses_same_threshold(self):
        minimum = 4 * 1024 ** 3
        free = [SimpleNamespace(free=minimum), SimpleNamespace(free=minimum - 1)]
        with patch.object(CROSS.Q, "DISK_FLOOR", minimum), patch.object(CROSS.shutil, "disk_usage", side_effect=free):
            with self.assertRaisesRegex(CROSS.Incomplete, f"required {minimum} bytes, available {minimum - 1} bytes"):
                self.run_link("disk-lost")
        self.assertEqual(self.receipt("disk-lost")["commands"], [])
        self.assertFalse((self.records / "cross-disk-lost-artifacts").exists())

    def test_core_collector_change_refuses_before_cross_command(self):
        with patch.object(CROSS.Q, "check_core_dumps", return_value={"collector_kind": "file", "limit_bytes": 0}):
            with self.assertRaisesRegex(CROSS.Incomplete, "core collector policy changed"):
                self.run_link("changed-collector")
        self.assertEqual(self.receipt("changed-collector")["commands"], [])
        self.assertFalse((self.records / "cross-changed-collector-artifacts").exists())

    def run_libtest_link(self, label, *, fault=None, relocated=False):
        """Drive the release libtest export using inert compiler output and PE/PDB bytes."""
        proc_hash = "0123456789abcdef"
        name = "nevrotick_windows_service_fixture"
        package = "nevrotick-windows-service-fixture"
        package_root = "/work/source/tools/windows-service-fixture"
        out = Path("target") / CROSS.TARGET / "release" / "build" / package / proc_hash / "out"
        image = out / (name + "-" + proc_hash + ".exe")
        symbols = image.with_suffix(".pdb")
        image_bytes = _image(3, pdb_path=symbols.name, guid=_UI_GUID)
        symbol_bytes = _pdb(guid=_OBSERVE_GUID if fault == "symbol-guid" else _UI_GUID)
        cargo = [
            str(self.stable / "bin/cargo"), "--verbose", "--verbose", "--config",
            f'target.{CROSS.TARGET}.linker="/work/linker"', "test", "--manifest-path",
            package_root + "/Cargo.toml", "--release", "--target", CROSS.TARGET,
            "--frozen", "--offline", "--jobs", "1", "--lib", "--no-run",
            "--message-format=json-render-diagnostics",
        ]
        emitted = "/work/" + str(image)
        artifact = {
            "reason": "compiler-artifact",
            "package_id": f"path+file://{package_root}#{package}@0.0.0",
            "manifest_path": package_root + "/Cargo.toml",
            "target": {"name": name, "kind": ["lib"], "crate_types": ["lib"],
                       "src_path": package_root + "/src/lib.rs", "edition": "2024", "test": True},
            "profile": {"opt_level": "3", "debug_assertions": False,
                        "overflow_checks": True, "debuginfo": 1, "test": True},
            "features": [], "fresh": False,
            "executable": emitted, "filenames": [emitted, "/work/" + str(symbols)],
        }

        def execute(root, work, command, **_kwargs):
            self.assertEqual(root, self.root)
            if command[0] != cargo[0]:
                self.assertEqual(command, [
                    str(self.llvm / "llvm-readobj"), "--file-headers", "--coff-load-config",
                    "--coff-debug-directory", "--coff-imports", emitted,
                ])
                return {"exit_code": 0, "failure": None, "output": "inspected"}
            self.assertEqual(command, cargo)
            (work / out).mkdir(parents=True)
            (work / image).write_bytes(image_bytes)
            (work / symbols).write_bytes(symbol_bytes)
            ledger = [str(self.llvm / "rust-lld"), "-flavor", "link"]
            ledger.extend("/libpath:" + str(self.libraries / scope) for scope in CROSS.LIBRARY_SCOPES)
            ledger.append("/OUT:" + (emitted if fault != "link-output" else "/work/other.exe"))
            (work / "linker-calls.jsonl").write_text(json.dumps(ledger) + "\n")
            if fault == "normal-library":
                artifact["profile"]["test"] = False
            if fault == "other-package":
                artifact["package_id"] += "-other"
            if fault == "other-path":
                artifact["executable"] = emitted.replace(proc_hash + ".exe", "f" * 16 + ".exe")
                artifact["filenames"] = [artifact["executable"]]
            rows = [artifact]
            if fault == "duplicate":
                rows.append(artifact)
            if fault != "missing-completion":
                rows.append({"reason": "build-finished", "success": True})
            return {"exit_code": 0, "failure": None, "output": "\n".join(map(json.dumps, rows)),
                    "command": ["sandbox", "--", *(reversed(command) if fault == "argv-order" else command)]}

        runtime = self.relocated_runtime(label, execute) if relocated else patch.object(
            CROSS.C.RUNTIME, "run", side_effect=execute,
        )
        with runtime:
            status = CROSS.run(SimpleNamespace(
                label=label, lane="libtest-link", source_checkpoint=self.checkpoint,
                sdk_config=self.sdk, sdk_inventory=self.sdk_inventory, vendor_dir=self.vendor,
                scratch_root=self.scratch if relocated else None,
            ))
        receipt = self.receipt(label)
        self.assertEqual(status, "passed")
        self.assertEqual(receipt["lane"], "libtest-link")
        self.assertTrue(receipt["bookends_passed"])
        self.assertFalse(receipt["native_execution"])
        self.assertEqual(len(receipt["commands"]), 2)
        self.assertEqual(set(receipt["artifacts"]), {name})
        export = receipt["artifacts"][name]
        self.assertNotIn("source_role", export)
        self.assertNotIn("selected_preentry_owner", export)
        self.role_admission.assert_not_called()
        self.assertEqual(Path(export["image"]).name, image.name)
        self.assertEqual(Path(export["pdb"]).name, symbols.name)
        self.assertEqual((self.root / export["image"]).read_bytes(), image_bytes)
        self.assertEqual((self.root / export["pdb"]).read_bytes(), symbol_bytes)
        self.assertEqual(export["pdb_identity"]["guid"], str(UUID(bytes_le=_UI_GUID)))

    def test_libtest_link_publishes_only_the_bound_hashed_artifact_pair(self):
        self.run_libtest_link("healthy-libtest")

    @contextlib.contextmanager
    def relocated_runtime(self, label, execute):
        """Receive the actual guarded dispatch while emitting an inert build result."""
        work = self.scratch / ("cross-" + label)

        def sandbox(root, observed_work, command, **options):
            self.assertEqual(root, self.root)
            self.assertEqual(observed_work, work)
            self.assertEqual(options["cwd"], work / "source")
            self.assertEqual(options["cpu_seconds"], 180)
            self.assertEqual(options["build_jobs"], 1)
            self.assertEqual(options["readonly"][0:2], [self.stable, self.vendor])
            self.assertEqual(options["sealed"], [work / "source", work / "cargo-home/config.toml", work / "linker"])
            return ["sandbox", "--", *command]

        def supervise(argv, *, cwd, timeout, environment, stdin, resource_guard):
            self.assertEqual(cwd, self.root)
            self.assertEqual(timeout, 180)
            self.assertEqual(environment, {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})
            self.assertEqual(stdin, CROSS.subprocess.DEVNULL)
            reason = resource_guard()
            if reason is not None:
                return {"exit_code": -9, "failure": "resource stop: " + reason,
                        "output": "", "command": argv}
            return dict(execute(cwd, work, argv[2:]), command=argv)

        with (patch.object(CROSS.C.RUNTIME, "sandbox_command", side_effect=sandbox),
              patch.object(CROSS.C.RUNTIME.ENV, "run", side_effect=supervise)):
            yield

    def test_relocated_libtest_keeps_export_and_receipt_in_checkout(self):
        self.run_libtest_link("relocated", relocated=True)
        receipt = self.receipt("relocated")
        self.assertEqual(receipt["work"], str(self.scratch / "cross-relocated"))
        self.assertEqual(receipt["limits"]["scratch_disk_floor_bytes"], CROSS.Q.DISK_FLOOR)
        self.assertFalse((self.cache / "cross-relocated").exists())

    def test_relocated_work_reserve_boundary_and_mid_command_stop(self):
        minimum = 4 * 1024 ** 3
        free = minimum - 1
        checks = []

        def disk(path):
            if path.is_relative_to(self.scratch):
                checks.append(path)
                # The third scratch sample is taken by the active command supervisor.
                return SimpleNamespace(free=minimum - 1 if len(checks) == 3 else free)
            return SimpleNamespace(free=minimum + 1024 * 1024)

        with (patch.object(CROSS.Q, "DISK_FLOOR", minimum),
              patch.object(CROSS.shutil, "disk_usage", side_effect=disk)):
            with self.assertRaisesRegex(CROSS.Incomplete, "before admission: WORK_DISK_RESERVE"):
                self.run_link("low-scratch", scratch_root=self.scratch)
            self.assertFalse(self.cache.exists())
            free = minimum
            checks.clear()
            with self.relocated_runtime("mid-stop", self.execute_fixture):
                with self.assertRaisesRegex(CROSS.Incomplete, "resource stop: WORK_DISK_RESERVE"):
                    self.run_link("mid-stop", scratch_root=self.scratch)
            receipt = self.receipt("mid-stop")
            self.assertEqual(receipt["status"], "incomplete")
            self.assertEqual(receipt["commands"][0]["failure"], "resource stop: WORK_DISK_RESERVE")
            self.assertFalse((self.records / "cross-mid-stop-artifacts").exists())

    def test_relocation_refuses_tool_vendor_and_sdk_aliases_before_work_creation(self):
        for scratch in (self.vendor, self.stable, self.libraries):
            scratch.chmod(0o700)
            with self.subTest(scratch=scratch):
                label = "overlap-" + scratch.name
                with self.assertRaisesRegex(CROSS.Incomplete, "overlaps a read-only input"):
                    self.run_link(label, scratch_root=scratch)
                self.assertFalse((scratch / ("cross-" + label)).exists())
                self.assertEqual(self.receipt(label)["commands"], [])
        alias = self.scratch.with_name("alias")
        alias.symlink_to(self.scratch)
        with self.assertRaisesRegex(CROSS.Incomplete, "without symlink redirection"):
            self.run_link("with-alias", scratch_root=alias)

    def test_receipt_created_before_lock_acquisition_preserves_prior_publication(self):
        label = "existing-after-lock"
        path = self.records / ("cross-" + label + ".json")
        prior = b'{"lane": "ready", "prior": true}\n'
        flock = CROSS.fcntl.flock

        def acquire(fd, operation):
            flock(fd, operation)
            path.write_bytes(prior)

        try:
            with (patch.object(CROSS.fcntl, "flock", side_effect=acquire),
                  self.relocated_runtime(label, self.execute_fixture)):
                with self.assertRaisesRegex(CROSS.Incomplete, "prior cross-link record already exists"):
                    self.run_link(label, scratch_root=self.scratch)
        finally:
            self.assertEqual(path.read_bytes(), prior)
            self.assertFalse((self.records / ("cross-" + label + "-artifacts")).exists())
            self.assertFalse((self.scratch / ("cross-" + label)).exists())
        self.assert_healthy("after-refusal")

    def test_relocated_publication_reserves_checkout_space_for_staged_bytes(self):
        minimum = 4 * 1024 ** 3

        def disk(path):
            return SimpleNamespace(free=minimum + (1 if path == self.root else 1024 * 1024))

        with (patch.object(CROSS.Q, "DISK_FLOOR", minimum),
              patch.object(CROSS.shutil, "disk_usage", side_effect=disk),
              self.relocated_runtime("no-copy", self.execute_fixture)):
            with self.assertRaisesRegex(CROSS.Incomplete, "disk reserve before artifact staging"):
                self.run_link("no-copy", scratch_root=self.scratch)
        self.assertFalse((self.records / "cross-no-copy-artifacts").exists())
        self.assertNotIn("artifacts", self.receipt("no-copy"))
        self.assertFalse(list(self.records.glob(".cross-publish-*")))

    def test_libtest_link_refuses_a_wrong_artifact_link_or_pdb(self):
        cases = {
            "normal-library": "source or release profile",
            "other-package": "source or release profile",
            "other-path": "output path",
            "duplicate": "one actual",
            "missing-completion": "one actual",
            "link-output": "linker output",
            "argv-order": "ordered Cargo command",
            "symbol-guid": "PDB GUID or age",
        }
        for fault, error in cases.items():
            with self.subTest(fault=fault), self.assertRaisesRegex(CROSS.Incomplete, error):
                self.run_libtest_link(fault, fault=fault)
            self.assertNotIn("artifacts", self.receipt(fault))
            self.assertFalse((self.records / ("cross-" + fault + "-artifacts")).exists())

    def test_same_basename_pdb_from_other_role_rejects_both_exports(self):
        self.assert_rejected_then_restored("wrong-role", self.pairs[ENGINE][1])

    def test_same_guid_wrong_age_rejects_both_exports(self):
        self.assert_rejected_then_restored("wrong-age", _pdb(guid=_OBSERVE_GUID, age=2))

    def test_original_symbols_must_survive_both_llvm_inspections(self):
        def drift_after_second_inspection(root, work, command, **kwargs):
            result = self.execute_fixture(root, work, command, **kwargs)
            if command[-1].endswith(BROKER + ".exe"):
                symbols = work / "target" / CROSS.TARGET / "release" / self.symbols[ENGINE]
                symbols.write_bytes(_pdb(guid=_UI_GUID, age=2))
            return result

        with patch.object(CROSS.C.RUNTIME, "run", side_effect=drift_after_second_inspection):
            with self.assertRaisesRegex(CROSS.Incomplete, "Copied or emitted artifact changed"):
                self.run_link("drift")
        self.assertFalse((self.records / "cross-drift-artifacts").exists())
        self.assertNotIn("artifacts", self.receipt("drift"))
        self.assert_healthy("drift-restored")

    def test_late_index_change_refuses_publication(self):
        hash_file = CROSS.C.PRODUCT.sha256
        changed = False

        def hash_staged(path):
            nonlocal changed
            if path.parent.parent.name.startswith(".cross-publish-"):
                changed = True
            return hash_file(path)

        def index():
            return dict(self.index, sha256="replacement") if changed else self.index

        with (patch.object(CROSS.C.PRODUCT, "sha256", side_effect=hash_staged),
              patch.object(CROSS.Q, "index_identity", side_effect=index)):
            with self.assertRaisesRegex(CROSS.Incomplete, "Staging bytes changed before cross-lane publication"):
                self.run_link("index-drift")
        self.assertTrue(changed)
        self.assertFalse((self.records / "cross-index-drift-artifacts").exists())
        self.assertNotIn("artifacts", self.receipt("index-drift"))
        self.assert_healthy("index-restored")

    def test_dangling_receipt_path_refuses_before_build(self):
        link = self.records / "cross-foreign.json"
        link.symlink_to(self.records / "missing.json")
        with self.assertRaisesRegex(CROSS.Incomplete, "prior cross-link record already exists"):
            self.run_link("foreign")
        self.assertTrue(link.is_symlink())
        self.assertFalse((self.cache / "cross-foreign").exists())
        self.assert_healthy("fresh-neighbor")


class ServiceSourcePrerequisites(unittest.TestCase):
    """Drive the receiver from actual-shaped, separately written graph and checkpoint receipts."""

    def setUp(self):
        scratch = tempfile.TemporaryDirectory()
        self.addCleanup(scratch.cleanup)
        self.records = Path(scratch.name)
        self.graph_path = self.records / "graph.json"
        self.checkpoint_path = self.records / "checkpoint.json"
        self.index = {"path": "/checkout/.git/index", "sha256": "index"}
        self.identity = {CROSS.Q.SELF: CROSS.C.PRODUCT.sha256(ROOT / CROSS.Q.SELF),
                         "service.rs": "product-source"}
        self.policy = {"name": "guarded-service-policy"}
        self.feature_delta = {"standalone_lock": "locked"}
        self.cargo = "/toolchain/bin/cargo"
        self.vendor = {"path": "/reviewed/vendor", "inventory_sha256": "vendor-bytes", "files": 2}
        self.fixture_names = ["integration::gnu_executable", "unit::fixture_control"]
        self.platform_names = ["unit::platform_control"]
        self.doc_names = ["crates/windows-platform/src/lib.rs - (line 9) - compile fail"]
        self.expected = {
            "fixture-test": CROSS.Q.canonical(self.fixture_names),
            "platform-test": CROSS.Q.canonical(self.platform_names),
            "platform-doc": CROSS.Q.canonical(self.doc_names),
            "fixture-doc": CROSS.Q.canonical([]),
        }
        for name, value in (
            ("RECORDS", self.records),
            ("source_identity", lambda: (self.policy, self.identity)),
            ("index_identity", lambda: self.index),
            ("manifest_admission", lambda policy: self.feature_delta),
            ("graph_row", lambda policy, target, *raw: {"target": target, "raw": list(raw)}),
        ):
            self.enterContext(patch.object(CROSS.Q, name, value))

    @staticmethod
    def command(argv, output=""):
        return {"argv": argv, "namespace_argv": argv, "exit_code": 0, "failure": None, "output": output}

    @staticmethod
    def successful_test_output(names):
        cases = "".join("test " + name + " ... ok\n" for name in names)
        return (cases + f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; "
                "0 measured; 0 filtered out; finished in 0.00s\n")

    def records_for_candidate(self):
        binding = {
            "candidate_sha256": CROSS.Q.canonical(self.identity),
            "recipe_sha256": self.identity[CROSS.Q.SELF],
            "config_sha256": "bf3fb733cfe6fb5967458dbedebabe113c6e322b137f88b9e920e796cf38815d",
            "feature_delta": self.feature_delta,
            "vendor": self.vendor,
            "tools": {"stable": "/reviewed/toolchain"},
        }
        common = {
            **binding,
            "schema_version": 1,
            "scope": "source-only inert service fixture",
            "status": "passed",
            "bookends_passed": True,
            "index_bytes_unchanged": True,
            "index_before": dict(self.index),
            "index_after": dict(self.index),
            "native_execution": False,
            "source_sha256": self.identity,
            "graph_binding": dict(binding),
        }
        graph_commands = [self.command(argv, f"observed result {index}") for index, argv in
                          enumerate(CROSS.Q.graph_commands(self.cargo))]
        graph = {**common, "lane": "graph", "commands": graph_commands, "graph_rows": [
            {"target": target, "raw": [row["output"] for row in graph_commands[index:index + 3]]}
            for index, target in zip((0, 3), CROSS.Q.TARGETS, strict=True)
        ]}
        test_commands = {
            tuple(CROSS.Q.lane_commands(lane, self.cargo)[0]): self.successful_test_output(names)
            for lane, names in (
                ("fixture-test", self.fixture_names), ("fixture-optimized", self.fixture_names),
                ("platform-test", self.platform_names), ("platform-optimized", self.platform_names),
                ("platform-doc", self.doc_names), ("fixture-doc", []),
            )
        }
        checkpoint = {**common, "lane": "privilege-checkpoint", "commands": [
            self.command(argv, test_commands.get(tuple(argv), "successful configured command"))
            for argv in CROSS.Q.lane_commands("privilege-checkpoint", self.cargo)
        ]}
        return graph, checkpoint

    def write_records(self, graph, checkpoint, *, replace_digest=None):
        self.graph_path.write_text(json.dumps(graph))
        graph_digest = hashlib.sha256(self.graph_path.read_bytes()).hexdigest()
        checkpoint["received_graph_path"] = str(self.graph_path)
        checkpoint["received_graph_sha256"] = replace_digest if replace_digest is not None else graph_digest
        self.checkpoint_path.write_text(json.dumps(checkpoint))
        return graph_digest

    def admit(self):
        # The small control roster is fixed independently of receipt stdout.
        with patch.object(CROSS, "SOURCE_TEST_ROSTERS", self.expected, create=True):
            return CROSS.source_prerequisites(self.checkpoint_path)

    def test_configured_graph_and_checkpoint_reach_current_named_case_admission(self):
        graph, checkpoint = self.records_for_candidate()
        graph_digest = self.write_records(graph, checkpoint)
        policy, identity, received, graph_hashes, checkpoint_digest = self.admit()
        self.assertEqual(policy, self.policy)
        self.assertEqual(identity, self.identity)
        self.assertEqual(received, checkpoint)
        self.assertEqual(graph_hashes, {str(self.graph_path): graph_digest})
        self.assertEqual(checkpoint_digest, hashlib.sha256(self.checkpoint_path.read_bytes()).hexdigest())

    def test_graph_digest_and_current_binding_are_required(self):
        graph, checkpoint = self.records_for_candidate()
        self.write_records(graph, checkpoint, replace_digest="wrong old digest")
        with self.assertRaisesRegex(CROSS.Incomplete, "graph receipt digest"):
            self.admit()
        graph["tools"] = {"stable": "/other/toolchain"}
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "graph.*binding"):
            self.admit()

    def test_graph_requires_six_actual_commands_in_configured_order(self):
        graph, checkpoint = self.records_for_candidate()
        graph["commands"] = graph["commands"][:4]
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "six actual"):
            self.admit()

    def test_missing_and_substituted_configured_checkpoint_commands_are_refused(self):
        graph, checkpoint = self.records_for_candidate()
        original = checkpoint["commands"].pop()
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "checkpoint commands"):
            self.admit()
        checkpoint["commands"].append({**original, "argv": ["/host/bin/cargo", *original["argv"][1:]]})
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "checkpoint command"):
            self.admit()

    def test_namespace_command_must_match_literal_configured_command(self):
        graph, checkpoint = self.records_for_candidate()
        graph_command = graph["commands"][0]
        graph_command["namespace_argv"] = ["/other/bin/cargo", *graph_command["argv"][1:]]
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "graph namespace command"):
            self.admit()
        graph, checkpoint = self.records_for_candidate()
        checkpoint["commands"][-1]["namespace_argv"] = [self.cargo, "check"]
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "checkpoint command"):
            self.admit()

    def test_same_size_substitution_and_ignored_case_are_refused(self):
        graph, checkpoint = self.records_for_candidate()
        commands = checkpoint["commands"]
        fixture_command = next(row for row in commands if row["argv"] ==
                               CROSS.Q.lane_commands("fixture-optimized", self.cargo)[0])
        fixture_command["output"] = self.successful_test_output([self.fixture_names[0], "unit::unknown_case"])
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "fixture-optimized.*roster"):
            self.admit()
        fixture_command["output"] = self.successful_test_output(self.fixture_names).replace("0 ignored", "1 ignored")
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "fixture-optimized.*skipped"):
            self.admit()

    def test_source_and_index_epochs_must_match_receipts(self):
        graph, checkpoint = self.records_for_candidate()
        checkpoint["source_sha256"] = {**self.identity, "service.rs": "some older product"}
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "current complete source"):
            self.admit()
        graph, checkpoint = self.records_for_candidate()
        graph["index_after"] = {**self.index, "sha256": "changed"}
        self.write_records(graph, checkpoint)
        with self.assertRaisesRegex(CROSS.Incomplete, "graph.*index"):
            self.admit()


if __name__ == "__main__":
    unittest.main()
