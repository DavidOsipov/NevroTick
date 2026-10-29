"""Independent evidence-admission controls; execute no compiler or Windows process."""
from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import json
import struct
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("allocation_observation_test", ROOT / "scripts/verify-time-domain-windows-allocation.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def snapshot(alloc=0, dealloc=0, live=100, peak=None, zeroed=0, realloc=0):
    """Synthetic independent integer counters, never actual product observations."""
    return {"alloc_requests": alloc, "zeroed_requests": zeroed, "realloc_requests": realloc,
            "requested_bytes": (alloc + zeroed + realloc) * 4096,
            "successful_requests": alloc + zeroed + realloc,
            "successful_requested_bytes": (alloc + zeroed + realloc) * 4096,
            "failed_requests": 0, "deallocations": dealloc, "live_payload_bytes": live,
            "peak_live_payload_bytes": max(live, live if peak is None else peak), "counter_error": False}


def metric(before, after):
    """Derive fixture deltas using independent raw arithmetic."""
    result = {name: after[name] - before[name] for name in MODULE.COUNTERS}
    return dict(result, live_baseline_bytes=before["live_payload_bytes"],
                retained_delta_bytes=after["live_payload_bytes"] - before["live_payload_bytes"],
                peak_above_baseline_bytes=after["peak_live_payload_bytes"] - before["live_payload_bytes"],
                before=copy.deepcopy(before), after=copy.deepcopy(after))


def box_control(base, product_calls):
    """Produce a known held/released positive-allocation control."""
    held = snapshot(base + 1, base, live=4196)
    dropped = snapshot(base + 1, base + 1, peak=4196)
    before = snapshot(base, base)
    return {"discriminates": True, "contents_match": True, "expected_payload_bytes": 4096,
            "product_calls": product_calls, "product_mismatches": 0,
            "held": metric(before, held), "after_product_while_held": metric(before, held),
            "after_explicit_drop": metric(before, dropped)}


def fixture():
    """A complete healthy evidence-shaped neighbor with independent literal counts."""
    before = snapshot(1, 1)
    zeroed = snapshot(1, 1, live=228, zeroed=1)
    grown = snapshot(1, 1, live=740, zeroed=1, realloc=1)
    shrunk = snapshot(1, 1, live=164, peak=740, zeroed=1, realloc=2)
    dropped = snapshot(1, 2, peak=740, zeroed=1, realloc=2)
    vector = {"discriminates": True, "contents_match": True,
              "zeroed": metric(before, zeroed), "grown": metric(before, grown),
              "shrunk": metric(before, shrunk), "after_explicit_drop": metric(before, dropped)}
    samples = []
    for case in MODULE.CASES:
        samples.append({"case": case, "phase": "cold", "calls": 1, "product_mismatches": 0,
                        "metrics": metric(snapshot(20, 20), snapshot(21, 21, peak=132))})
    for case in MODULE.CASES:
        for calls in (1, 32, 256, 4096):
            samples.append({"case": case, "phase": "warm_batch", "calls": calls, "product_mismatches": 0,
                            "metrics": metric(snapshot(30, 30), snapshot(30 + calls, 30 + calls, peak=132))})
    claims = {"candidate_sha256": "1" * 64, "observer_source_sha256": "2" * 64,
              "build_receipt_sha256": "3" * 64, "profile": "release"}
    value = {"schema_version": 1, "purpose": "test_only_time_domain_allocator_requests", "status": "observed",
             "runner_claims_unverified": claims,
             "build_observations": {"target": "x86_64-pc-windows-msvc", "debug_assertions": False,
                                    "external_build_identity_required": True},
             "workload": {"cases": 5, "cold_calls_per_case": 1, "warmups_per_case": 64,
                          "batches": [1, 32, 256, 4096], "held_box_product_calls": 32,
                          "total_product_calls": 22282, "warmup_mismatches": 0,
                          "cold_definition": "first_observation_per_case_in_shared_process_before_any_product_warmup"},
             "calibrations": {"before_product_box": box_control(0, 0),
                              "held_box_product_after_cold": box_control(22, 32),
                              "after_product_box": box_control(5000, 0), "safe_vec": vector},
             "samples": samples, "final_counters_before_output": snapshot(5001, 5001, peak=4196),
             "limits": ["artifact_specific_observed_global_requests", "allocation_elision_and_merging_permitted",
                        "black_box_best_effort", "logical_payload_peak_at_completed_callbacks",
                        "excludes_os_heap_overhead_caches_rss_and_native_realloc_transient_peak",
                        "excludes_allocations_outside_this_wrapper",
                        "no_universal_zero_allocation_or_absence_of_leaks_claim",
                        "no_appverifier_pageheap_or_release_qualification"]}
    return value, claims


def emitted_fixture(root):
    """Tiny locked control graph with both host proc macros and a Windows owner."""
    source, vendor = root / "source", root / "vendor"
    source.mkdir(); vendor.mkdir()
    (source / "crates/time-domain").mkdir(parents=True)
    (source / "crates/time-domain/Cargo.toml").write_text('[package]\nname="time-domain"\nversion="0.0.0"\n')
    dependencies = {"time-domain": ["garde"], "garde": ["garde_derive", "rustversion"],
                    "garde_derive": [], "rustversion": []}
    versions = {"time-domain": "0.0.0", "garde": "0.23.0", "garde_derive": "0.23.0", "rustversion": "1.0.23"}
    policy = {"packages": []}; records = []
    lock = 'version=4\n'
    for name, version in versions.items():
        lock += f'[[package]]\nname="{name}"\nversion="{version}"\ndependencies={json.dumps(dependencies[name])}\n'
        macro = name in ("garde_derive", "rustversion")
        if name != "time-domain":
            lock += 'source="registry+https://github.com/rust-lang/crates.io-index"\n'
            policy["packages"].append({"name": name, "version": version, "build_script": False})
            package = vendor / f"{name}-{version}"; package.mkdir()
            (package / "Cargo.toml").write_text(f'[package]\nname="{name}"\nversion="{version}"\n[lib]\nproc-macro={str(macro).lower()}\n')
        crate = name.replace('-', '_')
        folder = '/work/target/release/deps' if macro else '/work/target/x86_64-pc-windows-msvc/release/deps'
        filename = folder + '/lib' + crate + '-' + '1' * 16
        filenames = [filename + '.so'] if macro else [filename + '.rlib', filename + '.rmeta']
        if name == "time-domain": filenames[0] = '/work/target/x86_64-pc-windows-msvc/release/libtime_domain.rlib'
        records.append({"reason": "compiler-artifact",
                        "package_id": f"registry+https://github.com/rust-lang/crates.io-index#{name}@{version}" if name != "time-domain" else "path+file:///work/fixture/crates/time-domain#0.0.0",
                        "manifest_path": f"/vendor/{name}-{version}/Cargo.toml" if name != "time-domain" else "/work/fixture/crates/time-domain/Cargo.toml",
                        "target": {"name": crate, "kind": ["proc-macro" if macro else "lib"],
                                   "crate_types": ["proc-macro" if macro else "lib"],
                                   "src_path": f"/vendor/{name}-{version}/src/lib.rs" if name != "time-domain" else "/work/fixture/crates/time-domain/src/lib.rs"},
                        "profile": {"opt_level": "0" if macro else "3", "debug_assertions": False, "overflow_checks": True, "test": False},
                        "features": [], "filenames": filenames})
    (source / 'Cargo.lock').write_text(lock)
    return source, vendor, policy, records


class AllocationAdmissionTests(unittest.TestCase):
    def test_direct_rustc_explicitly_preserves_release_checks_and_debug_state(self):
        for profile, assertions in (("release", "no"), ("verify-release", "yes")):
            with self.subTest(profile=profile):
                self.assertEqual(MODULE.profile_flags(profile), [
                    "-C", "opt-level=3", "-C", "debug-assertions=" + assertions,
                    "-C", "overflow-checks=yes", "-C", "panic=unwind",
                    "-C", "debuginfo=1", "-C", "strip=none",
                    "-C", "lto=thin", "-C", "codegen-units=1",
                ])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_flags("unchecked")

    def test_direct_rustc_debug_profile_keeps_existing_codegen_settings(self):
        self.assertEqual(MODULE.profile_flags("debug"), [
            "-C", "opt-level=0", "-C", "debug-assertions=yes",
            "-C", "overflow-checks=yes", "-C", "panic=unwind",
            "-C", "debuginfo=2", "-C", "strip=none",
        ])

    def test_artifact_owner_profile_target_and_unique_library_are_mandatory(self):
        item = {"reason": "compiler-artifact", "manifest_path": "/work/fixture/crates/time-domain/Cargo.toml",
                "target": {"name": "time_domain", "kind": ["lib"], "crate_types": ["lib"]},
                "profile": {"opt_level": "3", "debug_assertions": False, "overflow_checks": True, "test": False},
                "filenames": ["/work/target/x86_64-pc-windows-msvc/release/libtime_domain.rlib"]}
        self.assertEqual(MODULE.artifact_path(json.dumps(item), "release"), item["filenames"][0])
        for change in ("profile", "owner", "duplicate", "target"):
            bad = copy.deepcopy(item)
            if change == "profile": bad["profile"]["overflow_checks"] = False
            if change == "owner": bad["manifest_path"] = "/work/fixture/crates/ntp-wire/Cargo.toml"
            if change == "target": bad["filenames"] = ["/work/target/debug/libtime_domain.rlib"]
            data = json.dumps(bad) + ("\n" + json.dumps(bad) if change == "duplicate" else "")
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.artifact_path(data, "release")

    def test_metrics_must_reconcile_not_merely_assert_zero_retention(self):
        good = metric(snapshot(1, 1), snapshot(4, 4, peak=228))
        self.assertEqual(MODULE.interval(good)["retained_delta_bytes"], 0)
        for change in ("retention", "counter_error", "decrease", "boolean", "missing"):
            bad = copy.deepcopy(good)
            if change == "retention": bad["after"]["live_payload_bytes"] += 1
            if change == "counter_error": bad["after"]["counter_error"] = True
            if change == "decrease": bad["after"]["deallocations"] = 0
            if change == "boolean": bad["alloc_requests"] = True
            if change == "missing": del bad["after"]["failed_requests"]
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.interval(bad)

    def test_complete_calibrated_observation_has_all_twenty_five_samples(self):
        value, claims = fixture()
        result = MODULE.validate_observation(value, claims)
        self.assertEqual(result["samples"], 25)
        self.assertEqual(result["product_calls"], 22282)
        self.assertEqual(result["warmup_retained_delta_bytes"], 0)

    def test_warmup_leak_cannot_hide_between_otherwise_stable_samples(self):
        value, claims = fixture()
        # Every measured batch remains locally stable; only its baseline moved.
        for sample in value["samples"][5:]:
            for endpoint in ("before", "after"):
                sample["metrics"][endpoint]["live_payload_bytes"] += 8
                sample["metrics"][endpoint]["peak_live_payload_bytes"] += 8
            sample["metrics"]["live_baseline_bytes"] += 8
        with self.assertRaisesRegex(MODULE.Incomplete, "warmup"):
            MODULE.validate_observation(value, claims)

    def test_missing_forged_or_nondiscriminating_evidence_does_not_pass(self):
        for change in ("empty", "wrong_calls", "identity", "calibration", "observed_retention", "product", "assertions"):
            value, claims = fixture()
            if change == "empty": value["samples"] = []
            if change == "wrong_calls": value["samples"][6]["calls"] = 31
            if change == "identity": value["runner_claims_unverified"]["candidate_sha256"] = "4" * 64; claims = dict(claims, candidate_sha256="1" * 64)
            if change == "calibration": value["calibrations"]["before_product_box"]["held"] = metric(snapshot(), snapshot())
            if change == "observed_retention": value["status"] = "observed_retained_delta"
            if change == "product": value["samples"][0]["product_mismatches"] = 1
            if change == "assertions": value["build_observations"]["debug_assertions"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_observation(value, claims)

    def test_duplicate_json_and_network_or_redirected_windows_paths_are_rejected(self):
        with self.assertRaises(MODULE.Incomplete): MODULE.decode(b'{"status":"passed","status":"failed"}')
        with self.assertRaises(MODULE.Incomplete): MODULE.decode(b'{"value":NaN}')
        self.assertEqual(MODULE.windows_path(Path('/mnt/c/Users/a/AppData/Local/NevroTick/test')),
                         'C:\\Users\\a\\AppData\\Local\\NevroTick\\test')
        for path in ('//server/share/file', '/tmp/owned.exe', '/mnt/c/Users/a:alternate/file'):
            with self.subTest(path=path), self.assertRaises(MODULE.Incomplete): MODULE.windows_path(Path(path))

    def test_actual_child_token_gate_cleanup_and_bounded_output_are_all_required(self):
        output, _ = fixture()
        token = {"user_sid": "S-1-5-21-10-20-30-1001", "elevated": False, "administrator": False, "elevation_type": 3}
        good = {"schema_version": 1, "status": "completed", "registry_writes": False,
                "image_settings_before": [], "image_settings_after": [], "image_sha256_before": "1" * 64,
                "image_sha256_after": "1" * 64, "supervisor_sha256": "2" * 64, "request_sha256": "3" * 64,
                "os": {"is_64_bit": True}, "runtime_inputs_unchanged": True, "adapter_sha256": "4" * 64,
                "runtime_files_sha256": {"csc.exe": "5" * 64},
                "execution": {"pid": 123, "exit_code": 0, "failure": None, "gate_sent": True,
                              "cleanup_complete": True, "output_overflow": False, "active_processes_before_cleanup": 0,
                              "active_processes_before_handle_release": 1, "process_handle_released": True,
                              "streams_released": True,
                              "active_processes_after_cleanup": 0, "cleanup_failure": None,
                              "stdin_encoding": "utf-8", "stdin_preamble_hex": "EF-BB-BF",
                              "stderr": "", "stdout": json.dumps(output), "parent_token": token, "child_token": copy.deepcopy(token)}}
        self.assertEqual(MODULE.validate_native_result(good, "1" * 64, "2" * 64, "3" * 64), output)
        for change in ("gate_sent", "cleanup_complete", "output_overflow", "token", "sid", "image", "settings", "exit", "zero_bool"):
            bad = copy.deepcopy(good)
            if change in ("gate_sent", "cleanup_complete"): bad["execution"][change] = False
            if change == "output_overflow": bad["execution"][change] = True
            if change == "token": bad["execution"]["child_token"]["elevated"] = True
            if change == "sid": bad["execution"]["child_token"]["user_sid"] += "-1"
            if change == "image": bad["image_sha256_after"] = "6" * 64
            if change == "settings": bad["image_settings_before"] = [{"key_present": True}]
            if change == "exit": bad["execution"]["exit_code"] = 1
            if change == "zero_bool": bad["execution"]["exit_code"] = False
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_native_result(bad, "1" * 64, "2" * 64, "3" * 64)
        # Releasing our process handle may change the native job count from one
        # to zero. Only the actual pre-termination zero authorizes a pass.
        for initial in (0, 1):
            healthy = copy.deepcopy(good)
            healthy["execution"]["active_processes_before_handle_release"] = initial
            self.assertEqual(MODULE.validate_native_result(healthy, "1" * 64, "2" * 64, "3" * 64), output)
        for key, replacement in (
                ("active_processes_before_handle_release", None),
                ("active_processes_before_handle_release", True),
                ("active_processes_before_handle_release", -1),
                ("process_handle_released", False),
                ("streams_released", False),
                ("streams_released", None),
                ("streams_released", 1),
                ("active_processes_before_cleanup", 1),
                ("active_processes_before_cleanup", False),
                ("active_processes_after_cleanup", 1),
                ("active_processes_after_cleanup", False),
                ("cleanup_failure", "original failure must survive cleanup"),
                ("stdin_encoding", "utf-16"),
                ("stdin_preamble_hex", ""),
                ("stdin_preamble_hex", "EF-BB-BF-00")):
            bad = copy.deepcopy(good)
            bad["execution"][key] = replacement
            with self.subTest(key=key, replacement=replacement), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_native_result(bad, "1" * 64, "2" * 64, "3" * 64)
        missing = copy.deepcopy(good)
        del missing["execution"]["streams_released"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_native_result(missing, "1" * 64, "2" * 64, "3" * 64)

    def test_hostile_json_booleans_do_not_substitute_for_executed_counts(self):
        for change in ("schema", "warmup_mismatches", "calls", "calibration_calls"):
            value, claims = fixture()
            if change == "schema": value["schema_version"] = True
            if change == "warmup_mismatches": value["workload"]["warmup_mismatches"] = False
            if change == "calls": value["samples"][0]["calls"] = True
            if change == "calibration_calls": value["calibrations"]["before_product_box"]["product_calls"] = False
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete): MODULE.validate_observation(value, claims)

    def test_sealed_dependency_mount_follows_writable_parent_for_each_direct_compiler(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            work = root / "work"; work.mkdir()
            sealed = work / "observer-deps"; sealed.mkdir()
            for compiler in ("clippy-driver", "rustc"):
                original = MODULE.PRODUCT.sandbox(root / "toolchain", work, root / "source", root / "vendor",
                                                   [compiler, "--emit=metadata"], None)
                invocation = MODULE.protect_dependencies(original, sealed)
                separator = invocation.index("--")
                header = invocation[:separator]
                parent = header.index(str(work))
                child = header.index(str(sealed))
                self.assertGreater(child, parent)
                self.assertEqual(header[child - 1:child + 2], ["--ro-bind", str(sealed), "/work/observer-deps"])
                self.assertEqual(header.count("/work/observer-deps"), 1)
                self.assertEqual(invocation[separator:], original[original.index("--"):])
                self.assertNotIn(str(sealed), original)

    def test_pe_flags_require_only_adopted_protections_and_report_terminal_server_awareness(self):
        with tempfile.TemporaryDirectory() as temporary:
            image = Path(temporary) / "observer.exe"
            data = bytearray(64 + 96)
            data[:2] = b"MZ"; struct.pack_into("<I", data, 60, 64)
            data[64:68] = b"PE\0\0"; struct.pack_into("<H", data, 68, 0x8664)
            struct.pack_into("<H", data, 88, 0x20b)
            for flags in (0x4160, 0xc160):
                struct.pack_into("<H", data, 158, flags); image.write_bytes(data)
                result = MODULE.pe_header(image)
                self.assertEqual(result["dll_characteristics"], flags)
                self.assertEqual(result["terminal_server_aware_observed"], bool(flags & 0x8000))
            for missing in (0x4000, 0x100, 0x40, 0x20):
                struct.pack_into("<H", data, 158, 0x4160 & ~missing); image.write_bytes(data)
                with self.subTest(missing=missing), self.assertRaises(MODULE.Incomplete): MODULE.pe_header(image)

    def test_actual_dependency_boundary_is_required_before_compiler_reliance(self):
        calls = []
        directory = Path('/owned/work/observer-deps')
        def run(label, command, **kwargs):
            calls.append((label, command, kwargs))
            return "ALLOCATION_DEPENDENCIES_READ_ONLY:3\n"
        MODULE.dependency_boundary(run, directory)
        self.assertEqual(calls, [("sealed-dependency-boundary", ["python3", "-c", MODULE.DEPENDENCY_BOUNDARY],
                                 {"readonly_dependencies": directory})])
        # Exit-zero text without the reached control marker is not evidence.
        with self.assertRaises(MODULE.Incomplete):
            MODULE.dependency_boundary(lambda *args, **kwargs: "", directory)
        self.assertIn("errno.EROFS", MODULE.DEPENDENCY_BOUNDARY)
        self.assertIn("library.chmod", MODULE.DEPENDENCY_BOUNDARY)

    def test_actual_host_macros_and_target_libraries_are_all_attributed_and_copied(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); source, vendor, policy, records = emitted_fixture(root)
            artifacts = MODULE.emitted_dependency_artifacts('\n'.join(map(json.dumps, records)), 'release', source, vendor, policy)
            macro_paths = [record['filenames'][0] for record in records if record['target']['kind'] == ['proc-macro']]
            self.assertEqual(len(artifacts), 6)
            self.assertTrue(all(path in artifacts for path in macro_paths))
            self.assertEqual({artifacts[path]['kind'] for path in macro_paths}, {'proc-macro'})
            work = root / 'work'; work.mkdir()
            for path in artifacts:
                original = work / path.removeprefix('/work/')
                original.parent.mkdir(parents=True, exist_ok=True); original.write_bytes(b'independent synthetic artifact ' + path.encode())
            destination = work / 'observer-deps'
            copied = MODULE.copy_dependency_artifacts(artifacts, work, destination)
            self.assertEqual(set(copied), {Path(path).name for path in artifacts})
            for path in artifacts:
                original, independent = work / path.removeprefix('/work/'), destination / Path(path).name
                self.assertEqual(original.read_bytes(), independent.read_bytes())
                self.assertNotEqual(original.stat().st_ino, independent.stat().st_ino)
                self.assertEqual(independent.stat().st_nlink, 1)
                self.assertEqual(independent.stat().st_mode & 0o777, 0o400)

    def test_missing_or_forged_host_artifact_attribution_is_not_accepted(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); source, vendor, policy, good = emitted_fixture(root)
            for change in ('missing_macro', 'wrong_root', 'wrong_kind', 'wrong_package', 'manifest', 'extra_file', 'duplicate'):
                records = copy.deepcopy(good)
                if change == 'missing_macro': records = [r for r in records if r['target']['name'] != 'garde_derive']
                if change == 'wrong_root': records[2]['filenames'][0] = '/work/target/x86_64-pc-windows-msvc/release/deps/libgarde_derive-' + '1' * 16 + '.so'
                if change == 'wrong_kind': records[2]['target']['kind'] = ['lib']; records[2]['target']['crate_types'] = ['lib']
                if change == 'wrong_package': records[2]['package_id'] = 'registry+https://github.com/rust-lang/crates.io-index#garde_derive@9.9.9'
                if change == 'manifest': records[2]['manifest_path'] = '/unreviewed/garde_derive/Cargo.toml'
                if change == 'extra_file': records[2]['filenames'].append('/etc/credentials.so')
                if change == 'duplicate': records.append(copy.deepcopy(records[2]))
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.emitted_dependency_artifacts('\n'.join(map(json.dumps, records)), 'release', source, vendor, policy)


if __name__ == "__main__":
    unittest.main()
