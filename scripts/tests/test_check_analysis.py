"""Independent admission/fail-closed controls; no installed analyzer is executed."""

import copy
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("check_analysis", ROOT / "scripts/check-analysis.py")
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


class AdmissionControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-analysis-tests-")
        self.addCleanup(self.temporary.cleanup)
        self.scratch = Path(self.temporary.name)
        self.checkout = self.scratch / "checkout"
        (self.checkout / "policy").mkdir(parents=True)
        self.base = self.scratch / "installation"
        (self.base / "common/bin").mkdir(parents=True)
        (self.base / "evidence").mkdir()
        self.home = self.scratch / "home"
        self.stable = self.home / f".rustup/toolchains/1.99.1-{CHECK.PLATFORM}"
        self.stable.mkdir(parents=True)
        self.pointer = self.scratch / "installation.json"
        self.pointer.write_text(json.dumps({"schema_version": 1, "root": str(self.base),
                                           "stable_toolchain": str(self.stable), "status": "installed",
                                           "platform": CHECK.PLATFORM}))
        self.pointer.chmod(0o600)
        self.policy = json.loads((ROOT / "policy/analysis-tools.json").read_text())
        self.policy["miri_sysroot"] = "common/miri-sysroot"
        self.policy["fuzz_target"] = CHECK.PLATFORM

    def write_policy(self):
        (self.checkout / "policy/analysis-tools.json").write_text(json.dumps(self.policy))

    def inventory(self, *, entry=None, scopes=None):
        target = self.base / "common/bin/analyzer"
        target.write_bytes(b"independent payload bytes")
        entries = {"bin/analyzer": entry or {"kind": "file", "bytes": target.stat().st_size,
                                            "sha256": hashlib.sha256(target.read_bytes()).hexdigest()}}
        value = {"schema_version": 1, "analysis_installation_root": str(self.base),
                 "tool_root": str(self.base / "common"), "scopes": scopes or ["bin"], "entries": entries}
        spec = {"name": "common", "path": "evidence/common.json", "root": "common", "scopes": scopes or ["bin"]}
        path = self.base / spec["path"]
        path.write_text(json.dumps(value))
        spec["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        return spec, value, path

    def check_inventory(self, specification):
        with patch.dict(CHECK.REQUIRED_SCOPES, {"common": {"bin"}}, clear=True):
            return CHECK.check_inventory(self.base, specification)

    def test_admitted_profiles_are_exact(self):
        CHECK.validate_policy(self.policy)
        for name in CHECK.PROFILES:
            for mutation in ([], self.policy["profiles"][name] + ["--unsafe"],
                             self.policy["profiles"][name][:-1], self.policy["profiles"][name] * 2):
                candidate = copy.deepcopy(self.policy)
                candidate["profiles"][name] = mutation
                with self.subTest(profile=name, mutation=mutation), self.assertRaises(CHECK.Incomplete):
                    CHECK.validate_policy(candidate)

    def test_tool_set_shipping_pin_codeql_and_private_compilers_cannot_drift(self):
        mutations = [lambda p: p.update(shipping_toolchain="nightly"),
                     lambda p: p.update(codeql="enabled"), lambda p: p["tools"].pop("loom"),
                     lambda p: p["tools"].update(extra={}),
                     lambda p: p["tools"]["lockbud"].update(toolchain="stable"),
                     lambda p: p["tools"]["miri"].update(sha256="0" * 64),
                     lambda p: p.update(miri_sysroot="/tmp/unchecked"),
                     lambda p: p.update(common_nightly_pin="nightly"),
                     lambda p: p.update(tools=[])]
        for mutate in mutations:
            candidate = copy.deepcopy(self.policy)
            mutate(candidate)
            with self.subTest(mutation=mutate), self.assertRaises(CHECK.Incomplete):
                CHECK.validate_policy(candidate)

    def test_duplicate_keys_and_nonfinite_json_are_rejected(self):
        for content in (b'{"entries":{"a":1,"a":2}}', b'{"profile":NaN}', b'\xff'):
            with self.subTest(content=content), self.assertRaises(CHECK.Incomplete):
                CHECK.decode(content)

    def test_traversal_and_noncanonical_paths_are_rejected(self):
        for path in (".", "../tool", "/tool", "a/../../tool", "a//tool", "a/./tool", "a\\tool", "a/", "", None):
            with self.subTest(path=path), self.assertRaises(CHECK.Incomplete):
                CHECK.relative(path)
        self.assertEqual(CHECK.relative("bin/analyzer"), "bin/analyzer")

    def test_private_pointer_healthy_control(self):
        with patch.object(Path, "home", return_value=self.home):
            self.assertEqual(CHECK.load_installation(self.pointer, self.checkout), (self.base, self.stable))

    def test_special_file_pointer_is_rejected_without_waiting_for_a_writer(self):
        fifo = self.scratch / "fifo-pointer"
        os.mkfifo(fifo, 0o600)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.read_once(fifo, private=True)

    def test_complete_fixture_snapshot_covers_nonrust_inputs_and_added_configuration(self):
        work = self.scratch / "work"
        project = work / "controls/future-project"
        project.mkdir(parents=True)
        (project / "native-input.h").write_text("#define EXPECTED 1\n")
        original = CHECK.input_snapshot(work, [work / "controls"])
        (project / "native-input.h").write_text("#define EXPECTED 0\n")
        self.assertNotEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (project / "native-input.h").write_text("#define EXPECTED 1\n")
        self.assertEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (project / ".cargo").mkdir()
        (project / ".cargo/config.toml").write_text('[alias]\ncompile="unsafe"\n')
        self.assertNotEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (work / ".cargo").mkdir()
        with self.assertRaises(CHECK.Incomplete):
            CHECK.input_snapshot(work, [work / "controls"])

    def test_mutable_store_root_symlink_is_rejected_before_any_traversal(self):
        work = self.scratch / "work"
        work.mkdir()
        outside = self.scratch / "outside"
        outside.mkdir()
        (outside / "private-canary").write_text("must never be enumerated or hashed")
        store = work / "vet-store"
        store.symlink_to(outside, target_is_directory=True)
        with patch.object(CHECK.os, "scandir") as enumerate_directory, \
             patch.object(CHECK, "digest") as hash_file, self.assertRaises(CHECK.Incomplete):
            CHECK.input_snapshot(work, [store], maximum_entries=4)
        enumerate_directory.assert_not_called()
        hash_file.assert_not_called()
        with patch.object(CHECK.os, "scandir") as enumerate_directory, \
             patch.object(CHECK, "digest") as hash_file, self.assertRaises(CHECK.Incomplete):
            CHECK.source_hashes(store)
        enumerate_directory.assert_not_called()
        hash_file.assert_not_called()
        with patch.object(CHECK.os, "scandir") as enumerate_directory, self.assertRaises(CHECK.Incomplete):
            CHECK.vet_store_snapshot(work, store)
        enumerate_directory.assert_not_called()
        store.unlink()
        store.mkdir()
        for name in ("config.toml", "audits.toml", "imports.lock", "unexpected"):
            (store / name).write_text("")
        with self.assertRaisesRegex(CHECK.Incomplete, "entry budget"):
            CHECK.input_snapshot(work, [store], maximum_entries=4)

    def test_bad_pointer_permissions_links_status_and_unpinned_stable_fail(self):
        original = self.pointer.read_text()
        for mode in (0o644, 0o400, 0o777):
            self.pointer.chmod(mode)
            with self.subTest(mode=mode), patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
                CHECK.load_installation(self.pointer, self.checkout)
        self.pointer.chmod(0o600)
        for key, value in (("status", "installing"), ("platform", "windows"),
                           ("root", str(self.base / ".." / "installation")), ("stable_toolchain", str(self.base))):
            candidate = json.loads(original)
            candidate[key] = value
            self.pointer.write_text(json.dumps(candidate))
            with self.subTest(key=key), patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
                CHECK.load_installation(self.pointer, self.checkout)
        self.pointer.write_text(original)
        alias = self.scratch / "pointer-alias"
        alias.symlink_to(self.pointer)
        with patch.object(Path, "home", return_value=self.home), self.assertRaises(OSError):
            CHECK.load_installation(alias, self.checkout)
        alias.unlink()
        alias.hardlink_to(self.pointer)
        with patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
            CHECK.load_installation(self.pointer, self.checkout)

    def test_complete_inventory_healthy_control_and_drift(self):
        specification, _, _ = self.inventory()
        report, covered = self.check_inventory(specification)
        self.assertEqual(report["entries"], 1)
        self.assertEqual(covered, {"common/bin/analyzer"})
        target = self.base / "common/bin/analyzer"
        target.write_bytes(b"substituted executable")
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)

    def test_inventory_hash_is_checked_before_json_parse(self):
        specification, _, path = self.inventory()
        path.write_text("unparseable substituted bytes")
        with patch.object(CHECK, "decode") as parse, self.assertRaisesRegex(CHECK.Incomplete, "SHA256 mismatch"):
            self.check_inventory(specification)
        parse.assert_not_called()

    def test_inventory_rejects_added_missing_and_special_entries(self):
        for case in ("extra", "missing", "fifo"):
            specification, _, _ = self.inventory()
            target = self.base / "common/bin/analyzer"
            if case == "extra":
                (target.parent / "injected.so").write_text("extra")
            elif case == "missing":
                target.unlink()
            else:
                target.unlink()
                os.mkfifo(target)
            with self.subTest(case=case), self.assertRaises((CHECK.Incomplete, OSError)):
                self.check_inventory(specification)
            for item in target.parent.iterdir():
                item.unlink()

    def test_inventory_cannot_omit_required_scope_or_add_traversal(self):
        specification, _, _ = self.inventory()
        for scopes in ([], ["bin", "bin"], ["bin", "bin/subdir"], ["../common/bin"], ["bin", "../escape"]):
            candidate = copy.deepcopy(specification)
            candidate["scopes"] = scopes
            with self.subTest(scopes=scopes), self.assertRaises(CHECK.Incomplete):
                self.check_inventory(candidate)

    def test_symlink_must_match_text_and_remain_in_selected_root_and_declared_scope(self):
        specification, value, path = self.inventory()
        link = self.base / "common/bin/link"
        link.symlink_to("analyzer")
        value["entries"]["bin/link"] = {"kind": "symlink", "target": "analyzer",
                                       "resolved_analysis_relative": "ignored audit metadata"}
        path.write_text(json.dumps(value))
        specification["sha256"] = CHECK.digest(path)
        self.assertEqual(self.check_inventory(specification)[0]["entries"], 2)
        link.unlink()
        link.symlink_to("./analyzer")
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)
        link.unlink()
        external = self.base / "kani/payload"
        external.parent.mkdir()
        external.write_bytes(b"admitted elsewhere, forbidden selected-root escape")
        link.symlink_to(external)
        value["entries"]["bin/link"]["target"] = str(external)
        path.write_text(json.dumps(value))
        specification["sha256"] = CHECK.digest(path)
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)

    def test_duplicate_inventory_entry_with_correct_outer_hash_is_rejected(self):
        specification, _, path = self.inventory()
        original = path.read_text()
        path.write_text(original.replace('"entries": {', '"entries": {"duplicate": {}, "duplicate": {},', 1))
        specification["sha256"] = CHECK.digest(path)
        with self.assertRaisesRegex(CHECK.Incomplete, "Duplicate JSON key"):
            self.check_inventory(specification)

    def test_unsafe_policy_or_absent_inventory_prevents_all_execution(self):
        with patch.object(CHECK.RUNTIME, "run") as run, patch.object(Path, "home", return_value=self.home):
            self.policy["profiles"]["miri_flags"] = ["-Zmiri-disable-isolation"]
            self.write_policy()
            with self.assertRaises(CHECK.Incomplete):
                CHECK.admit(self.checkout, self.pointer)
            self.policy["profiles"] = copy.deepcopy(CHECK.PROFILES)
            self.policy["inventories"] = []
            self.write_policy()
            with self.assertRaisesRegex(CHECK.Incomplete, "inventories"):
                CHECK.admit(self.checkout, self.pointer)
        run.assert_not_called()

    def test_fresh_external_private_receipt_cannot_overwrite_or_redirect(self):
        output = self.scratch / "receipt.json"
        with CHECK.receipt_file(output, self.checkout) as receipt:
            receipt.write("independent receipt")
        self.assertEqual(output.stat().st_mode & 0o777, 0o600)
        with self.assertRaises(FileExistsError):
            CHECK.receipt_file(output, self.checkout)
        alias = self.scratch / "alias.json"
        alias.symlink_to(output)
        with self.assertRaises(OSError):
            CHECK.receipt_file(alias, self.checkout)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.receipt_file(self.checkout / "report.json", self.checkout)
        public = self.scratch / "public"
        public.mkdir(mode=0o755)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.receipt_file(public / "report.json", self.checkout)

    def test_actual_nonzero_and_custody_failures_cannot_masquerade_as_detections(self):
        for result in ({"exit_code": 0, "failure": None, "output": "Undefined Behavior"},
                       {"exit_code": 1, "failure": "timeout", "output": "Undefined Behavior"},
                       {"exit_code": 1, "failure": None, "output": "compiler setup failed"}):
            with self.subTest(result=result), self.assertRaises(CHECK.Incomplete):
                CHECK.require_result(result, negative="Undefined Behavior")
        CHECK.require_result({"exit_code": 1, "failure": None, "output": "Undefined Behavior"}, negative="Undefined Behavior")

    def kani_report(self, defect=False):
        checks = [{"category": "assertion", "status": "Failure" if defect else "Success",
                   "description": "assertion failed: u32::from(ns) == wide"}]
        checks += [{"category": "cover", "status": "Satisfied"} for _ in range(3)]
        return {"verification_results": {"summary": {"status": "completed", "executed": 1,
                    "total_harnesses": 1, "successful": 0 if defect else 1, "failed": 1 if defect else 0},
                "results": [{"harness_id": "conversion_is_exact_or_rejected", "status": "Failure" if defect else "Success", "checks": checks}]}}

    def test_kani_requires_actual_harness_nonempty_checks_and_intended_assertion(self):
        for defect in (False, True):
            self.assertEqual(CHECK.kani_counts(self.kani_report(defect), defect)["harnesses"], 1)
            for mutate in (lambda p: p["summary"].update(executed=0),
                           lambda p: p.update(results=[]),
                           lambda p: p["results"][0].update(checks=[]),
                           lambda p: p["results"][0].update(harness_id="other")):
                report = self.kani_report(defect)
                mutate(report["verification_results"])
                with self.subTest(defect=defect), self.assertRaises(CHECK.Incomplete):
                    CHECK.kani_counts(report, defect)
        wrong = self.kani_report(True)
        wrong["verification_results"]["results"][0]["checks"][0]["description"] = "unrelated compiler crash"
        with self.assertRaises(CHECK.Incomplete):
            CHECK.kani_counts(wrong, True)

    def test_default_main_never_runs_fixture_or_accepts_previous_receipt(self):
        admitted = {"base": self.base, "policy_sha256": "a" * 64, "inventories": []}
        with patch.object(CHECK, "admit", return_value=admitted), \
             patch.object(CHECK, "fixtures") as fixtures, patch("sys.stdout", new_callable=io.StringIO) as stdout:
            self.assertEqual(CHECK.main([]), 0)
        fixtures.assert_not_called()
        report = json.loads(stdout.getvalue())
        self.assertEqual(report["mode"], "admission-only")
        self.assertEqual(report["product_qualification"], "pending")
        existing = self.scratch / "old.json"
        existing.write_text('{"status":"passed"}')
        with patch.object(CHECK, "admit") as admit, patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(CHECK.main(["--json", str(existing)]), 1)
        admit.assert_not_called()
        self.assertEqual(existing.read_text(), '{"status":"passed"}')

    def test_failed_fixture_retains_raw_partial_receipt(self):
        admitted = {"base": self.base, "policy_sha256": "a" * 64, "inventories": []}
        output = self.scratch / "failed.json"

        def failed(admission, selected, progress):
            progress({"work": "/tmp/owned-control", "lanes": [{"exit_code": 1, "output": "setup failed", "failure": None}]})
            raise CHECK.Incomplete("independent setup failure")

        with patch.object(CHECK, "admit", return_value=admitted), patch.object(CHECK, "fixtures", side_effect=failed), \
             patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(CHECK.main(["--fixtures", "--tool", "kani", "--json", str(output)]), 1)
        report = json.loads(output.read_text())
        self.assertEqual(report["status"], "incomplete")
        self.assertEqual(report["lanes"][0]["output"], "setup failed")

    def test_vet_requires_success_without_exemptions_and_actual_missing_loom_review(self):
        healthy = {"conclusion": "success", "vetted_fully": [], "vetted_partially": [], "vetted_with_exemptions": []}
        rejected = {"conclusion": "fail (vetting)", "failures": [{"name": "loom", "version": "0.7.2",
                                                                    "missing_criteria": ["safe-to-deploy"]}]}
        self.assertEqual(CHECK.vet_counts(healthy, False)["exemptions"], 0)
        self.assertEqual(CHECK.vet_counts(rejected, True)["missing_audits"], 1)
        for candidate, defect in (({}, False), (dict(healthy, vetted_with_exemptions=["fake"]), False),
                                  (dict(rejected, failures=[]), True),
                                  ({"conclusion": "fail (vetting)", "failures": [{"name": "other"}]}, True)):
            with self.subTest(candidate=candidate), self.assertRaises(CHECK.Incomplete):
                CHECK.vet_counts(candidate, defect)

    def test_vet_control_store_rejects_even_unused_audits_imports_or_exemptions(self):
        store = self.scratch / "vet-store"
        store.mkdir()
        defaults = {"config.toml": '[cargo-vet]\nversion="0.10"\n',
                    "audits.toml": "[audits]\n", "imports.lock": "# empty imports\n"}
        for name, text in defaults.items():
            (store / name).write_text(text)
        CHECK.empty_vet_store(store)
        for name, addition in (("config.toml", '\n[exemptions]\nunused=[]\n'),
                               ("config.toml", '\n[imports.unused]\nurl="https://example.invalid"\n'),
                               ("audits.toml", '\n[[audits.unused]]\nversion="1.0.0"\n'),
                               ("imports.lock", '\n[unused]\n')):
            (store / name).write_text(defaults[name] + addition)
            with self.subTest(name=name, addition=addition), self.assertRaises(CHECK.Incomplete):
                CHECK.empty_vet_store(store)
            (store / name).write_text(defaults[name])
        (store / "extra.toml").write_text("# forbidden additional policy\n")
        with self.assertRaises(CHECK.Incomplete):
            CHECK.empty_vet_store(store)

    def test_miri_all_four_seeds_require_counts_even_with_interleaved_summaries(self):
        interleaved = "".join(f"Trying seed: {seed}\n" for seed in range(4))
        interleaved += "running 1 test\n" * 4
        interleaved += "test result: " * 4 + "ok" * 4 + ". 1 passed; 0 failed; 0 ignored;" * 4
        self.assertEqual(CHECK.miri_count(interleaved), 4)
        for output in (interleaved.replace("Trying seed: 3", "Trying seed: 2"),
                       interleaved.replace("running 1 test", "running 0 tests", 1),
                       interleaved.replace(". 1 passed;", ". 0 passed;", 1),
                       interleaved + "panicked", ""):
            with self.subTest(output=output), self.assertRaises(CHECK.Incomplete):
                CHECK.miri_count(output)

    def test_fixture_launches_use_pinned_fixed_commands_and_independent_counts(self):
        (self.base / "common/cargo/registry/cache").mkdir(parents=True)
        (self.base / "common/cargo/registry/index").mkdir()
        (self.base / "common/cargo/registry/src").mkdir()
        source = CHECK.CONTROLS.create(self.base / "common/controls-source")
        for project in source.values():
            (project / "Cargo.lock").write_text("version = 4\n")
        (self.base / self.policy["common_nightly"]).mkdir(parents=True)
        admission = {"base": self.base, "root": self.checkout, "stable": self.stable, "policy": self.policy}
        calls = []

        def run(root, work, command, **options):
            calls.append((command, options))
            self.assertEqual(options["readonly"], [self.base, self.stable])
            self.assertEqual(set(path.name for path in options["sealed"][:4]), {"controls", "cache", "index", "src"})
            self.assertFalse(any(key.startswith("LD_") for key in options["environment"]))
            output, exit_code = "", 0
            if "test" in command:
                if any("healthy_owned_value" in item for item in command):
                    self.assertNotIn("RUSTC", options["environment"])
                    self.assertNotIn("RUSTDOC", options["environment"])
                    self.assertEqual(options["environment"]["MIRIFLAGS"], " ".join(CHECK.PROFILES["miri_flags"]))
                    output = "".join(f"Trying seed: {seed}\nrunning 1 test\n" for seed in range(4))
                    output += "test result: ok. 1 passed; 0 failed; 0 ignored;\n" * 4
                elif "seeded_use_after_free" in command:
                    output, exit_code = "Undefined Behavior: pointer has been freed", 1
                elif "tests::seeded_lost_update" in command:
                    output, exit_code = "both increments must survive", 101
                else:
                    output = "test result: ok. 1 passed; 0 failed; 0 ignored;"
            elif "fuzz" in command and "build" in command:
                self.assertEqual(command[command.index("--target") + 1], CHECK.PLATFORM)
                self.assertNotIn("--jobs", command)
                binary = work / Path(command[command.index("--target-dir") + 1]).relative_to("/work") / "x86_64-unknown-linux-gnu/release" / command[3]
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_text("emitted analyzer fixture")
            elif command[0].startswith("/work/target-fuzz-"):
                self.assertEqual(options["mode"], "sanitizer")
                for flag in CHECK.PROFILES["fuzzer_flags"]:
                    self.assertEqual(command.count(flag), 1)
                if command[0].endswith("/defect"):
                    output, exit_code = "signed byte decoding must preserve negative offsets", 77
                else:
                    output = "Done 256 runs in 0 second(s)"
            elif "vet" in command:
                self.assertEqual(options["environment"]["CARGO"], str(self.stable / "bin/cargo"))
                self.assertNotIn("check", command)
                if command[-1] == "init":
                    self.assertEqual(command[1:], ["vet", "init"])
                    supply = options["cwd"] / "supply-chain"
                    supply.mkdir()
                    (supply / "audits.toml").write_text("# empty owned fixture\n[audits]\n")
                    (supply / "config.toml").write_text('[cargo-vet]\nversion="0.10"\n')
                    (supply / "imports.lock").write_text("# empty imports\n")
                elif "unreviewed" in str(options["cwd"]):
                    self.assertIn("--store-path", command)
                    store = work / Path(command[command.index("--store-path") + 1]).relative_to("/work")
                    self.assertNotIn(store / "audits.toml", options["sealed"])
                    self.assertNotIn(store / "imports.lock", options["sealed"])
                    self.assertNotIn(store / "config.toml", options["sealed"])
                    self.assertIn(work / "controls", options["sealed"])
                    output = json.dumps({"conclusion": "fail (vetting)", "failures": [{"name": "loom", "version": "0.7.2",
                                                                                       "missing_criteria": ["safe-to-deploy"]}]})
                    exit_code = 255
                else:
                    self.assertIn("--store-path", command)
                    output = json.dumps({"conclusion": "success", "vetted_fully": [], "vetted_partially": [], "vetted_with_exemptions": []})
            elif "--harness" in command:
                defect = "kani-defect" in str(options["cwd"])
                exported = command[command.index("--export-json") + 1]
                (work / Path(exported).relative_to("/work")).write_text(json.dumps(self.kani_report(defect)))
                if defect:
                    output, exit_code = "VERIFICATION:- FAILED", 1
            elif "lockbud" in command:
                self.assertIn("--frozen", command)
                self.assertIn("--lib", command)
                if "lockbud-defect" in str(options["cwd"]):
                    output, exit_code = '"DoubleLock"', 101
                else:
                    target = work / Path(command[command.index("--target-dir") + 1]).relative_to("/work")
                    (target / "proof.rmeta").write_text("fresh analyzed library")
                    output = "Compiling nevrotick_lock_control"
            else:
                self.fail("Unexpected command: " + str(command))
            return {"command": command, "exit_code": exit_code, "output": output, "failure": None}

        state = {}
        try:
            with patch.object(CHECK.RUNTIME, "run", side_effect=run):
                result = CHECK.fixtures(admission, CHECK.TOOLS, state.update)
            self.assertEqual(len(result["lanes"]), 15)
            self.assertEqual(len(calls), 15)
            self.assertTrue(all(lane["sources_unchanged"] for lane in result["lanes"]))
            self.assertEqual([lane["observed_runs"] for lane in result["lanes"] if "observed_runs" in lane], [256])
            self.assertEqual([lane["observed_tests"] for lane in result["lanes"] if "observed_tests" in lane], [4, 1])
            stores = [lane["vet_store_lock_exception"] for lane in result["lanes"] if "vet_store_lock_exception" in lane]
            self.assertEqual(len(stores), 2)
            self.assertTrue(all(store["unchanged"] for store in stores))
        finally:
            if state.get("work"):
                shutil.rmtree(state["work"])

        def corrupt_store(root, work, command, **options):
            observed = run(root, work, command, **options)
            if "--store-path" in command:
                store = work / Path(command[command.index("--store-path") + 1]).relative_to("/work")
                (store / "config.toml").write_text('[exemptions]\nfabricated=[]\n# injected exemption\n')
            return observed

        corrupted = {}
        try:
            with patch.object(CHECK.RUNTIME, "run", side_effect=corrupt_store), self.assertRaises(CHECK.Incomplete):
                CHECK.fixtures(admission, ("vet",), corrupted.update)
            self.assertFalse(corrupted["lanes"][-1]["vet_store_lock_exception"]["unchanged"])
            self.assertIn("file size changed", corrupted["lanes"][-1]["vet_store_lock_exception"]["guard_error"])
            self.assertIn("--store-path", corrupted["lanes"][-1]["command"])
            self.assertEqual(json.loads(corrupted["lanes"][-1]["output"])["conclusion"], "success")
        finally:
            if corrupted.get("work"):
                shutil.rmtree(corrupted["work"])


if __name__ == "__main__":
    unittest.main()
