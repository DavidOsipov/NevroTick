"""Independent exact graph/seed/feature controls; no compiler or package code runs."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from types import SimpleNamespace
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("time_analysis_test", ROOT / "scripts/verify-time-domain-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
PURE_REGISTRY_NAMES = frozenset({
    "castaway", "cfg-if", "compact_str", "garde", "garde_derive", "itoa",
    "proc-macro2", "quote", "rustversion", "ryu", "smallvec",
    "static_assertions", "syn", "unicode-ident",
})


class DomainAnalysisTests(unittest.TestCase):
    """Valid neighbors discriminate missing graph checks and meaningless seeds."""

    def source_parity_fixture(self):
        """Own small real source/archive pins; native code and Cargo never execute."""
        temporary = tempfile.TemporaryDirectory(prefix="domain-source-parity-")
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name) / "product"
        base = Path(temporary.name) / "installation"
        (root / "policy").mkdir(parents=True)
        original = json.loads((ROOT / "policy/ntp-wire-analysis.json").read_text())["coverage_guided_fuzzing"]
        selected = json.loads((ROOT / "policy/time-domain-analysis.json").read_text())["coverage_guided_fuzzing"]
        product_policy = json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())
        product_text = (ROOT / "Cargo.lock").read_text()
        expected_text = (ROOT / "policy/time-domain-fuzz.lock").read_text()

        def file_map(directory):
            return {path.relative_to(directory).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                    for path in directory.rglob("*") if path.is_file()}

        for entry in original["locked_registry_packages"]:
            name, version = entry["name"], entry["version"]
            directory = base / "common/cargo/registry/src/fixture" / f"{name}-{version}"
            directory.mkdir(parents=True)
            manifest = (f'[package]\nname={json.dumps(name)}\nversion={json.dumps(version)}\n'
                        f'license={json.dumps(entry["declared_license"])}\n')
            if entry["build_script"] is not False:
                manifest += "build=" + json.dumps(entry["build_script"]) + "\n"
                (directory / "build.rs").write_text("// Fixture only; never compiled.\n")
            (directory / "Cargo.toml").write_text(manifest)
            (directory / "src").mkdir()
            (directory / "src/lib.rs").write_text("//! Fixture only; never compiled.\n")
            if name == "libfuzzer-sys":
                (directory / "libfuzzer").mkdir()
                native = directory / "libfuzzer/Fixture.cpp"
                native.write_text("// Independently pinned control fixture; never compiled.\n")
                original["native_files"] = {"Fixture.cpp": {"sha256": hashlib.sha256(native.read_bytes()).hexdigest()}}
            archive = base / "common/cargo/registry/cache/fixture" / f"{name}-{version}.crate"
            archive.parent.mkdir(parents=True, exist_ok=True)
            archive.write_bytes(("Unexecuted source-parity archive fixture: " + name).encode())
            previous = entry["checksum"]
            entry["checksum"] = hashlib.sha256(archive.read_bytes()).hexdigest()
            expected_text = expected_text.replace(previous, entry["checksum"])
            files_hash = hashlib.sha256(json.dumps(file_map(directory), sort_keys=True, separators=(",", ":")).encode()).hexdigest()
            entry["source_files_sha256"] = files_hash
            if name in PURE_REGISTRY_NAMES:
                product_text = product_text.replace(previous, entry["checksum"])
                admitted = next(item for item in product_policy["packages"] if item["name"] == name)
                admitted.update(checksum=entry["checksum"], files_sha256=files_hash)
        control_names = {item["name"] for item in original["locked_registry_packages"]}
        control_blocks = []
        for raw in expected_text.split("[[package]]")[1:]:
            block = "[[package]]\n" + raw.strip() + "\n"
            if tomllib.loads(block)["package"][0]["name"] in control_names:
                control_blocks.append(block)
        control = base / "common/controls-source/fuzz/Cargo.lock"
        control.parent.mkdir(parents=True)
        control.write_text("version = 4\n\n" + "\n".join(control_blocks))
        original["control_lock_sha256"] = hashlib.sha256(control.read_bytes()).hexdigest()
        notice = root / original["llvm_notice"]
        notice.parent.mkdir(parents=True, exist_ok=True)
        notice.write_text("Independent control notice fixture.\n")
        original["llvm_notice_sha256"] = hashlib.sha256(notice.read_bytes()).hexdigest()
        (root / "policy/ntp-wire-analysis.json").write_text(json.dumps({"coverage_guided_fuzzing": original}))
        (root / "Cargo.lock").write_text(product_text)
        lock = root / selected["lockfile"]
        lock.parent.mkdir(parents=True, exist_ok=True)
        lock.write_text(expected_text)
        selected["locked_registry_packages"] = [item for item in tomllib.loads(expected_text)["package"] if "source" in item]
        session = SimpleNamespace(base=base, vendor=Path(temporary.name) / "vendor",
                                  dependency_policy=product_policy, vendor_identity={"qualified-vendor": "a" * 64})
        return root, session, selected, original

    def test_source_parity_keeps_unrelated_runtime_and_original_control_independent(self):
        root, session, selected, original = self.source_parity_fixture()
        self.assertIn("tokio", {item["name"] for item in session.dependency_policy["packages"]})
        main_libc = next(item for item in session.dependency_policy["packages"] if item["name"] == "libc")
        control_libc = next(item for item in original["locked_registry_packages"] if item["name"] == "libc")
        self.assertNotEqual(main_libc["checksum"], control_libc["checksum"])
        with patch.object(MODULE, "ROOT", root), \
                patch.object(MODULE.PRODUCT, "admit_vendor", return_value=session.vendor_identity) as vendor:
            self.assertEqual(MODULE.admit_fuzz_sources(session, selected), original)
            vendor.assert_called_once_with(session.vendor, session.dependency_policy)

    def test_actual_domain_closure_uses_only_fourteen_validated_selected_records(self):
        policy = json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())
        with patch.object(MODULE.PRODUCT, "validated_dependency_records",
                          wraps=MODULE.PRODUCT.validated_dependency_records) as validation:
            products = MODULE.admitted_domain_products((ROOT / "Cargo.lock").read_text(), policy)
        validation.assert_called_once_with(policy, include_metadata=False)
        self.assertEqual(set(products), PURE_REGISTRY_NAMES)
        self.assertTrue(set(products).isdisjoint({"tokio", "getrandom", "libc", "r-efi", "wasi"}))

    def test_reached_registry_nodes_require_selected_role_and_exact_identity(self):
        policy = json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())
        garde = next(item for item in policy["packages"] if item["name"] == "garde")

        def block(item, dependencies=()):
            result = ('[[package]]\nname=' + json.dumps(item["name"]) +
                      '\nversion=' + json.dumps(item["version"]) +
                      '\nsource=' + json.dumps(MODULE.REGISTRY) +
                      '\nchecksum=' + json.dumps(item["checksum"]) + '\n')
            if dependencies:
                result += "dependencies=" + json.dumps(list(dependencies)) + "\n"
            return result

        local = 'version=4\n[[package]]\nname="time-domain"\nversion="0.0.0"\ndependencies=["garde"]\n'
        healthy = local + block(garde)
        self.assertEqual(set(MODULE.admitted_domain_products(healthy, policy)), {"garde"})
        metadata = next(item for item in policy["metadata_only_packages"] if item["name"] == "r-efi")
        unselected = {"name": "unreviewed-child", "version": "1.0.0", "checksum": "f" * 64}
        variants = {
            "missing-locked-node": local + block(garde, ["missing"]),
            "duplicate-locked-node": healthy + block(garde),
            "wrong-version": local + block({**garde, "version": "0.23.1"}),
            "wrong-checksum": local + block({**garde, "checksum": "f" * 64}),
            "unselected-node": local + block(garde, [unselected["name"]]) + block(unselected),
            "metadata-only-node": local + block(garde, [metadata["name"]]) + block(metadata),
            "duplicate-local-owner": healthy + local[local.index("[[package]]"):],
            "changed-local-edge": healthy.replace('dependencies=["garde"]', 'dependencies=["tokio"]'),
        }
        for change, text in variants.items():
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admitted_domain_products(text, policy)
        for change in ("missing-selected-record", "duplicate-selected-record", "metadata-role-promotion"):
            bad = json.loads(json.dumps(policy))
            if change == "missing-selected-record":
                bad["packages"] = [item for item in bad["packages"] if item["name"] != "garde"]
            elif change == "duplicate-selected-record":
                bad["packages"].append(dict(garde))
            else:
                bad["packages"].append(dict(metadata))
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admitted_domain_products(healthy, bad)

    def test_actual_reviewed_fuzz_lock_remains_the_exact_existing_twenty_three_package_bytes(self):
        expected = (ROOT / "policy/time-domain-fuzz.lock").read_text()
        original = json.loads((ROOT / "policy/ntp-wire-analysis.json").read_text())["coverage_guided_fuzzing"]
        names = {item["name"] for item in original["locked_registry_packages"]}
        blocks = []
        for raw in expected.split("[[package]]")[1:]:
            block = "[[package]]\n" + raw.strip() + "\n"
            if tomllib.loads(block)["package"][0]["name"] in names:
                blocks.append(block)
        control = "version = 4\n\n" + "\n".join(blocks)
        self.assertEqual(MODULE.reviewed_fuzz_lock((ROOT / "Cargo.lock").read_text(), control), expected)

    def test_source_parity_preserves_both_pure_and_original_control_grant_checks(self):
        for name, diagnostic in (("garde", "Garde product graph/grant parity"),
                                 ("libc", "Original fuzz identity/grant parity")):
            root, session, selected, _original = self.source_parity_fixture()
            selected["selected_grants"][name] = "BSD-3-Clause"
            with self.subTest(name=name), patch.object(MODULE, "ROOT", root), \
                    patch.object(MODULE.PRODUCT, "admit_vendor", return_value=session.vendor_identity), \
                    self.assertRaisesRegex(MODULE.Incomplete, diagnostic):
                MODULE.admit_fuzz_sources(session, selected)

    def test_source_parity_preserves_original_archive_and_exact_ten_record_checks(self):
        for change in ("archive-bytes", "missing-control-record", "duplicate-control-record"):
            root, session, selected, original = self.source_parity_fixture()
            if change == "archive-bytes":
                libc = next(item for item in original["locked_registry_packages"] if item["name"] == "libc")
                archive = session.base / "common/cargo/registry/cache/fixture" / f"libc-{libc['version']}.crate"
                archive.write_bytes(b"changed after source admission")
                diagnostic = "Original fuzz source/archive"
            else:
                if change == "missing-control-record":
                    original["locked_registry_packages"].pop()
                else:
                    original["locked_registry_packages"].append(dict(original["locked_registry_packages"][0]))
                (root / "policy/ntp-wire-analysis.json").write_text(json.dumps({"coverage_guided_fuzzing": original}))
                diagnostic = "Original control source cohort"
            with self.subTest(change=change), patch.object(MODULE, "ROOT", root), \
                    patch.object(MODULE.PRODUCT, "admit_vendor", return_value=session.vendor_identity), \
                    self.assertRaisesRegex(MODULE.Incomplete, diagnostic):
                MODULE.admit_fuzz_sources(session, selected)

    def test_reviewed_lock_preserves_exact_registry_blocks_and_real_local_nodes(self):
        product = 'version=4\n[[package]]\nname="time-domain"\nversion="0.0.0"\ndependencies=["garde"]\n'
        product += '[[package]]\nname="garde"\nversion="0.23.0"\nsource="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="garde-hash"\n'
        control = 'version=4\n[[package]]\nname="nevrotick-fuzz-control"\nversion="0.0.0"\n'
        control += '[[package]]\nname="libfuzzer-sys"\nversion="0.4.13"\nsource="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="fuzz-hash"\n'
        text = MODULE.reviewed_fuzz_lock(product, control)
        packages = {p["name"]: p for p in tomllib.loads(text)["package"]}
        self.assertEqual(set(packages), {"garde", "libfuzzer-sys", "time-domain", "time-domain-fuzz"})
        self.assertEqual(packages["garde"]["checksum"], "garde-hash")
        self.assertEqual(packages["time-domain"]["dependencies"], ["garde"])
        self.assertEqual(packages["time-domain-fuzz"]["dependencies"], ["libfuzzer-sys", "time-domain"])
        conflicted = control + product[product.index('[[package]]\nname="garde"'):].replace('garde-hash', 'tampered')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.reviewed_fuzz_lock(product, conflicted)

    def test_seeded_cases_include_exact_ties_negative_eras_and_rounding_limits(self):
        seeds = MODULE.fuzz_seeds()
        for name in ["ordinary-symmetric", "randomized-wire-token", "asymmetric-path", "era-rollover-explicit",
                     "era-negative-unique", "era-negative-none", "era-negative-ambiguous", "era-half-shaped",
                     "phase-tie-positive", "phase-tie-negative", "rtt-tie-positive", "rtt-tie-negative",
                     "phase-i64-upper", "phase-i64-upper-rejected", "rtt-i64-lower", "rtt-i64-lower-rejected",
                     "clock-add-overflow", "coordinate-below-minimum", "coordinate-maximum"]:
            self.assertIn(name, seeds)
        self.assertTrue(all(len(data) == 176 for data in seeds.values()))
        coordinate = lambda name, start: int.from_bytes(seeds[name][start:start + 16], "big", signed=True)
        self.assertEqual(coordinate("coordinate-below-minimum", 0), -(1 << 95) - 1)
        self.assertEqual(coordinate("coordinate-maximum", 0), (1 << 95) - 1)
        self.assertEqual(coordinate("era-negative-unique", 120), -(1 << 64))
        self.assertEqual(coordinate("phase-i64-upper", 64), 79228162514264337589)
        self.assertEqual(coordinate("phase-i64-upper-rejected", 64), 79228162514264337590)

    def test_metadata_checks_sources_features_and_exact_reachable_dependency_edges(self):
        registry = "registry+https://github.com/rust-lang/crates.io-index"
        selected = {"locked_registry_packages": [{"name": "garde", "version": "0.23.0", "source": registry}],
                    "gnu_reachable_features": {"garde": ["derive"]},
                    "gnu_dependency_edges": {"time-domain-fuzz": ["time-domain"], "time-domain": ["garde"], "garde": []}}
        packages = [{"id": n, "name": n, "version": "0.23.0" if n == "garde" else "0.0.0",
                     "source": registry if n == "garde" else None} for n in ["garde", "time-domain", "time-domain-fuzz"]]
        nodes = [{"id": n, "features": ["derive"] if n == "garde" else [],
                  "deps": [{"pkg": d} for d in selected["gnu_dependency_edges"][n]]}
                 for n in ["garde", "time-domain", "time-domain-fuzz"]]
        metadata = {"packages": packages, "resolve": {"root": "time-domain-fuzz", "nodes": nodes}}
        self.assertEqual(MODULE.fuzz_metadata(metadata, selected)["reachable_features"]["garde"], ["derive"])
        for changed in ["feature", "source", "edge", "duplicate"]:
            bad = json.loads(json.dumps(metadata))
            if changed == "feature": bad["resolve"]["nodes"][0]["features"].append("full")
            if changed == "source": bad["packages"][0]["source"] = "git+https://unreviewed"
            if changed == "edge": bad["resolve"]["nodes"][2]["deps"] = [{"pkg": "garde"}]
            if changed == "duplicate": bad["packages"].append(dict(bad["packages"][0]))
            with self.subTest(changed=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.fuzz_metadata(bad, selected)

    def test_four_retained_seeds_preserve_literal_saved_transmit_and_unfolded_inputs(self):
        fixture = json.loads((ROOT / "tasks/s1.1-exchange-vectors.json").read_text())
        seeds = MODULE.fuzz_seeds()
        for case in fixture["cases"]:
            if case["id"] not in {"ordinary-symmetric", "asymmetric-path", "randomized-wire-token", "era-rollover-explicit"}:
                continue
            context = {**fixture["default_context"], **case["context_overrides"]}
            packet = bytes.fromhex(case["packet_hex"])
            values = [int(context["real_t1_hex"], 16), int.from_bytes(packet[32:40], "big"),
                      int.from_bytes(packet[40:48], "big"), int(context["local_t4_hex"], 16)]
            if case["id"] == "era-rollover-explicit":
                values[1:] = [value + (1 << 64) for value in values[1:]]
            actual = [int.from_bytes(seeds[case["id"]][start:start + 16], "big", signed=True) for start in (48, 64, 80, 96)]
            self.assertEqual(actual, values)
            self.assertEqual(int.from_bytes(seeds[case["id"]][112:120], "big"), int.from_bytes(packet[32:40], "big"))
            a, b, c, d = actual
            expected = case["independent_arithmetic"]
            self.assertEqual((b - a + c - d) * expected["offset_seconds_denominator"],
                             expected["offset_seconds_numerator"] * (1 << 33))
            self.assertEqual((d - a - c + b) * expected["signed_rtt_seconds_denominator"],
                             expected["signed_rtt_seconds_numerator"] * (1 << 32))

    def test_actual_standalone_lock_has_23_exact_registry_and_two_true_local_nodes(self):
        lock = tomllib.loads((ROOT / "policy/time-domain-fuzz.lock").read_text())["package"]
        registry = [item for item in lock if "source" in item]
        self.assertEqual(len(registry), 23)
        self.assertEqual({item["name"] for item in lock if "source" not in item}, {"time-domain", "time-domain-fuzz"})
        product = {item["name"]: item for item in tomllib.loads((ROOT / "Cargo.lock").read_text())["package"]}
        old = {item["name"]: item for item in json.loads((ROOT / "policy/ntp-wire-analysis.json").read_text())["coverage_guided_fuzzing"]["locked_registry_packages"]}
        for item in registry:
            if item["name"] in PURE_REGISTRY_NAMES:
                self.assertEqual(item, product[item["name"]])
            if item["name"] in old:
                self.assertEqual({key: item[key] for key in ("name", "version", "source", "checksum")},
                                 {key: old[item["name"]][key] for key in ("name", "version", "source", "checksum")})

    def test_contract_dependencies_are_real_independent_immutable_copies(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); deps = work / "target/debug/deps"; deps.mkdir(parents=True)
            original = work / "target/debug/libtime_domain.rlib"; original.write_text("actual library")
            (deps / "libgarde.rlib").write_text("actual dependency")
            (deps / "unrelated.d").write_text("not a compiler dependency")
            protected = []
            session = SimpleNamespace(work=work, protect=lambda path: protected.append(path))
            copied = MODULE.copy_contract_dependencies(session, "/work/target/debug/libtime_domain.rlib")
            self.assertEqual(set(item.name for item in copied.iterdir()), {"libtime_domain.rlib", "libgarde.rlib"})
            self.assertNotEqual(original.stat().st_ino, (copied / "libtime_domain.rlib").stat().st_ino)
            self.assertEqual((copied / "libtime_domain.rlib").stat().st_mode & 0o777, 0o400)
            self.assertEqual(protected, [copied])
            original.write_text("later changed build output")
            self.assertEqual((copied / "libtime_domain.rlib").read_text(), "actual library")

    def test_contract_specimens_cover_each_private_type_and_mixed_clock_direction(self):
        cases = MODULE.contract_examples()
        self.assertIsNone(cases["legal"]["code"])
        for name in ["coordinate", "window", "monotonic", "awake", "mono_duration", "awake_duration",
                     "phase", "rtt", "exchange", "rounded_phase", "rounded_rtt"]:
            for suffix, code in [("construction", "E0451"), ("mutation", "E0616"), ("default", "E0277")]:
                self.assertEqual(cases[name + "_" + suffix]["code"], code)
        for name in ["monotonic_awake", "awake_monotonic", "utc_monotonic", "utc_awake",
                     "monotonic_awake_duration", "awake_monotonic_duration", "raw_exchange", "rounded_units"]:
            self.assertEqual(cases[name]["code"], "E0308")

    def test_policy_rejects_lowered_threshold_missing_owner_and_changed_source(self):
        healthy = MODULE.read_policy(ROOT)
        self.assertEqual(len(healthy["critical_files"]), 9)
        # This minimal graph/threshold fixture has no product checkout. Exact E
        # source/candidate/proof admission is tested separately on real owners.
        fixture_policy = json.loads(json.dumps(healthy))
        fixture_policy["mutation_equivalences"] = []
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            inputs = [MODULE.POLICY, "policy/verification-manifest.toml",
                      *healthy["coverage_guided_fuzzing"]["inputs_sha256"]]
            for name in inputs:
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, target)
            (root / MODULE.POLICY).write_text(json.dumps(fixture_policy))
            self.assertEqual(MODULE.read_policy(root)["coverage_minimum_line_percent"], 90)
            policy = root / MODULE.POLICY
            bad = json.loads(policy.read_text()); bad["coverage_minimum_line_percent"] = 89
            policy.write_text(json.dumps(bad))
            with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)
            policy.write_text(json.dumps(fixture_policy))
            inventory = root / "policy/verification-manifest.toml"
            original = inventory.read_text()
            inventory.write_text(original.replace('"crates/time-domain/src/era.rs"', '"crates/time-domain/src/missing.rs"'))
            with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)
            inventory.write_text(original)
            source = root / "policy/time-domain-fuzz-target.rs"; source.write_text(source.read_text() + "\n// changed\n")
            with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)

    def test_exact_equivalence_admission_is_unavoidable_in_domain_policy(self):
        with patch.object(MODULE.COMMON, "admit_mutation_equivalences",
                          side_effect=MODULE.Incomplete("unadmitted source-bound proof")) as admission:
            with self.assertRaisesRegex(MODULE.Incomplete, "source-bound proof"):
                MODULE.read_policy(ROOT)
            self.assertEqual(admission.call_count, 1)
            self.assertEqual(admission.call_args.args[:2], (ROOT, MODULE.SCOPE))


class DomainLeapAnalysisTests(unittest.TestCase):
    """Independent executable-owner and causal-consumer runner controls."""

    BUNDLED_ERROR = (
        "{ let _ = (CONTINUOUS_FROM, DATA_REVISION, FINITE_FUTURE_BEFORE, "
        "HISTORY_COMPLETE_BEFORE, LEAP_RECORDS, PROVENANCE_SHA256, RAW_SHA256, "
        "LeapContinuousFrom::new, LeapDataIdentity::new, LeapFutureExpiry::new, "
        "LeapHistoryBefore::new); LeapError::InvalidRecords }"
    )

    EXPECTED = {
        "coordinate": "crates/time-domain/src/coordinate.rs",
        "era": "crates/time-domain/src/era.rs",
        "exchange": "crates/time-domain/src/exchange.rs",
        "clock": "crates/time-domain/src/clock.rs",
        "rounding": "crates/time-domain/src/rounding.rs",
        "leap": "crates/time-domain/src/leap.rs",
        "leap_validation": "crates/time-domain/src/leap/validation.rs",
        "leap_interpretation": "crates/time-domain/src/leap/interpretation.rs",
        "leap_bundled": "crates/time-domain/src/leap/bundled.rs",
    }

    def policy_fixture(self):
        temporary=tempfile.TemporaryDirectory(prefix="leap-analysis-policy-")
        self.addCleanup(temporary.cleanup)
        root=Path(temporary.name)
        policy=MODULE.read_policy(ROOT)
        for name in (MODULE.POLICY,"policy/verification-manifest.toml",
                     *policy["coverage_guided_fuzzing"]["inputs_sha256"]):
            target=root/name;target.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(ROOT/name,target)
        return root,policy

    def test_nine_exact_executable_owners_each_require_nonempty_coverage(self):
        self.assertEqual(MODULE.SCOPE.select("all"),self.EXPECTED)
        manifest=tomllib.loads((ROOT/"policy/verification-manifest.toml").read_text())
        for name,filename in self.EXPECTED.items():
            with self.subTest(owner=name):
                self.assertEqual(manifest["critical_files"].count(filename),1)
                self.assertEqual(sum(filename in row["files"] for row in manifest["security_modules"]),1)
                mutant={"package":"time-domain","file":filename,"name":filename+":literal-control"}
                self.assertEqual(MODULE.COMMON.mutation_inventory(MODULE.SCOPE,[mutant],512,filename),
                                 {mutant["name"]:mutant})
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.mutation_inventory(MODULE.SCOPE,[],512,filename)
                wrong={**mutant,"package":"foreign-package"}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.mutation_inventory(MODULE.SCOPE,[wrong],512,filename)
                result={"data":[{"files":[{"filename":"/work/source/"+filename,
                        "summary":{"lines":{"count":10,"covered":9}}}]}]}
                self.assertEqual(MODULE.COMMON.coverage_counts(MODULE.SCOPE,result,
                                 "/work/source/"+filename)["line_percent"],90)
                result["data"][0]["files"][0]["summary"]["lines"]["covered"]=8
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.coverage_counts(MODULE.SCOPE,result,"/work/source/"+filename)
                result["data"][0]["files"][0]["summary"]["lines"]={"count":0,"covered":0}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.coverage_counts(MODULE.SCOPE,result,"/work/source/"+filename)
        generated="crates/time-domain/src/leap/generated.rs"
        self.assertNotIn(generated,manifest["critical_files"])
        self.assertFalse(any(generated in row["files"] for row in manifest["security_modules"]))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.SCOPE.selected_file(generated)

    def test_old_contracts_preserved_and_leap_private_traits_and_borrow_are_selected(self):
        cases=MODULE.contract_examples()
        for old in ("legal","coordinate_construction","window_default","rounded_rtt_mutation",
                    "coordinate_from","monotonic_awake","raw_exchange"):
            self.assertIn(old,cases)
        expected={"leap_legal","leap_records_borrow_mutation","leap_metadata_units"}
        for prefix in ("leap_history","leap_expiry","leap_continuous","leap_identity",
                       "leap_scope","leap_knowledge","leap_answer"):
            expected.update(prefix+"_"+suffix for suffix in ("construction","mutation","default","from"))
        self.assertTrue(expected <= set(cases),expected-set(cases))
        self.assertIsNone(cases["leap_legal"]["code"])
        for name in expected-{"leap_legal"}:
            case=cases[name]
            self.assertTrue(case["tokens"])
            self.assertIn("Box<dyn std::error::Error>",case["source"])
            self.assertEqual(case["line"],4)
            self.assertTrue(case["source"].splitlines()[3].strip())
            self.assertEqual(len(case["source"].splitlines()),6)
            self.assertNotIn("\\\\n",case["source"])
        self.assertEqual(cases["leap_records_borrow_mutation"]["code"],"E0506")
        self.assertEqual(cases["leap_metadata_units"]["code"],"E0308")
        self.assertIn("records[0].1",cases["leap_records_borrow_mutation"]["source"].splitlines()[3])
        self.assertIn("knowledge.records()",cases["leap_records_borrow_mutation"]["source"].splitlines()[3])
        self.assertIn("bundled_leap_knowledge()",cases["leap_legal"]["source"])

    def test_schema_four_profile_caps_and_legacy_fuzz_scope_are_closed(self):
        root,healthy=self.policy_fixture()
        self.assertEqual(healthy["schema_version"],4)
        self.assertEqual(healthy["source_file_count"],166)
        self.assertEqual(healthy["tool_admission_profile"],"stable-core")
        self.assertEqual(healthy["jobs"],6)
        self.assertEqual(healthy["test_threads"],12)
        self.assertEqual(healthy["mutation_jobs"],10)
        self.assertEqual(healthy["mutation_process_limit"],512)
        self.assertEqual(healthy["mutation_message_format"],"json")
        self.assertEqual(healthy["mutation_equivalences"],[])
        self.assertEqual(healthy["mutation_error_values"],{"leap_bundled":[self.BUNDLED_ERROR]})
        self.assertEqual(healthy["coverage_guided_fuzzing"]["subjects"],
                         dict(list(self.EXPECTED.items())[:5]))
        variants=[]
        for field,value in (("schema_version",1),("schema_version",2),("schema_version",3),("source_file_count",True),("maximum_mutants",513),
                            ("jobs",12),("jobs",True),("test_threads",6),
                            ("mutation_jobs",12),("mutation_process_limit",513),
                            ("mutation_message_format","human"),("coverage_minimum_line_percent",89),
                            ("mutation_minimum_viable_killed_percent",94),("tool_admission_profile","full"),
                            ("mutation_error_values",{"leap":["InvalidRecords"]}),
                            ("mutation_equivalences",[{}]),("unexpected",1)):
            bad=json.loads(json.dumps(healthy));bad[field]=value;variants.append(bad)
        for value in ({},None,{"leap":["InvalidRecords"]},
                      {"leap_bundled":["LeapError::InvalidRecords"]},
                      {"leap_bundled":[self.BUNDLED_ERROR.replace("LeapHistoryBefore::new","LeapHistoryBefore::new(0)")]},
                      {"leap_bundled":[self.BUNDLED_ERROR.replace("LeapError::InvalidRecords","crate::LeapError::InvalidRecords")]},
                      {"leap_bundled":[self.BUNDLED_ERROR+"; panic!()"]},
                      {**healthy["mutation_error_values"],"coordinate":["crate::LeapError::InvalidRecords"]},
                      {**healthy["mutation_error_values"],"leap_validation":["crate::LeapError::InvalidRecords"]},
                      {**healthy["mutation_error_values"],"leap":["crate::LeapError::InvalidRecords; panic!()"]},
                      {**healthy["mutation_error_values"],"leap":["crate::LeapError::InvalidMetadata"]},
                      {**healthy["mutation_error_values"],"leap":"crate::LeapError::InvalidRecords"},
                      {**healthy["mutation_error_values"],"leap":[True]},
                      {**healthy["mutation_error_values"],"leap":["crate::LeapError::InvalidRecords"]*2}):
            bad=json.loads(json.dumps(healthy));bad["mutation_error_values"]=value;variants.append(bad)
        bad=json.loads(json.dumps(healthy));bad["critical_files"].pop();variants.append(bad)
        bad=json.loads(json.dumps(healthy))
        bad["coverage_guided_fuzzing"]["subjects"]["leap"]=self.EXPECTED["leap"];variants.append(bad)
        for bad in variants:
            with self.subTest(delta=[k for k in bad if bad[k]!=healthy.get(k)]):
                (root/MODULE.POLICY).write_text(json.dumps(bad))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root)
        (root/MODULE.POLICY).write_text(json.dumps(healthy))
        self.assertEqual(MODULE.read_policy(root),healthy)

    def test_exact_error_literal_reaches_only_bundled_subject_command(self):
        policy=MODULE.read_policy(ROOT)
        for name,filename in self.EXPECTED.items():
            expected=["--error",self.BUNDLED_ERROR] if name=="leap_bundled" else []
            with self.subTest(owner=name):
                self.assertEqual(MODULE.COMMON.per_subject_mutation_error_args(
                    MODULE.SCOPE,filename,policy),expected)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.per_subject_mutation_error_args(
                MODULE.SCOPE,"crates/time-domain/src/leap/generated.rs",policy)

    def test_new_context_cannot_claim_all_or_fuzz_or_fallback_full_tools(self):
        policy=MODULE.read_policy(ROOT)
        for lane in ("all","fuzz"):
            args=SimpleNamespace(lane=lane,subject="all",tool_admission_profile="stable-core")
            with self.subTest(lane=lane),patch.object(MODULE.COMMON,"Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute(args,{})
                session.assert_not_called()
        args=SimpleNamespace(lane="contracts",subject="all",tool_admission_profile="full")
        with patch.object(MODULE.COMMON,"Session") as session:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.execute(args,{})
            session.assert_not_called()
        for lane in ("contracts","coverage","mutation"):
            self.assertEqual(MODULE.COMMON.admission_profile(
                SimpleNamespace(lane=lane,tool_admission_profile="stable-core"),
                MODULE.SCOPE,policy),"stable-core")

    def test_selected_lane_reports_leap_unfuzzed_and_preserves_finish(self):
        args=SimpleNamespace(lane="coverage",subject="leap_validation",
                             tool_admission_profile="stable-core")
        with tempfile.TemporaryDirectory() as temporary:
            session=SimpleNamespace(work=Path(temporary),tools=Path(temporary)/"tools",
                                    run=lambda *args,**kwargs:None,finish=unittest.mock.Mock())
            report={}
            with patch.object(MODULE.COMMON,"Session",return_value=session), \
                 patch.object(MODULE.COMMON,"run_analysis_lanes") as run, \
                 patch.object(MODULE,"fuzz",side_effect=AssertionError("unselected fuzz")):
                MODULE.execute(args,report)
            self.assertEqual(report["subjects"],{"leap_validation":self.EXPECTED["leap_validation"]})
            self.assertEqual(report["fuzz"]["status"],"not-performed")
            self.assertEqual(report["fuzz"]["not_fuzzed"],dict(list(self.EXPECTED.items())[5:]))
            self.assertEqual(run.call_args.args[-1],"coverage")
            session.finish.assert_called_once()
            session.finish.reset_mock()
            with patch.object(MODULE.COMMON,"Session",return_value=session), \
                 patch.object(MODULE.COMMON,"run_analysis_lanes",
                              side_effect=MODULE.Incomplete("independent lane failure")):
                with self.assertRaisesRegex(MODULE.Incomplete,"independent lane failure"):
                    MODULE.execute(args,{})
            session.finish.assert_called_once()

    def test_contract_dispatch_requires_both_actual_legal_markers(self):
        cases=MODULE.contract_examples()
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);dependency=work/"immutable";dependency.mkdir()
            calls=[]
            def run(label,command,timeout,**kwargs):
                calls.append((label,kwargs.get("specimen")))
                if label == "contract-library":
                    self.assertEqual(command[command.index("--jobs")+1],"6")
                if label.startswith("external-legal-execution"):
                    return cases[Path(command[0]).name]["success_marker"]
                return ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=run,protect=lambda _:None)
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",return_value="/work/target/debug/libtime_domain.rlib"), \
                 patch.object(MODULE,"copy_contract_dependencies",return_value=dependency):
                MODULE.contracts(session,{"contracts_wall_seconds":300,"jobs":6},report)
            self.assertEqual(report["contracts"]["legal_consumers_compiled_and_executed"],2)
            self.assertEqual(len([label for label,_ in calls if label.startswith("external-legal-execution")]),2)
            negatives=[specimen for _,specimen in calls if specimen is not None]
            self.assertEqual(len(negatives),len(cases)-2)
            self.assertTrue(all(case["source"] and case["tokens"] for case in negatives))
        # A legacy success marker cannot substitute for the new actual consumer.
        with tempfile.TemporaryDirectory() as temporary:
            work=Path(temporary);dependency=work/"immutable";dependency.mkdir()
            def wrong_marker(label,command,timeout,**kwargs):
                return cases["legal"]["success_marker"] if label.startswith("external-legal-execution") else ""
            session=SimpleNamespace(work=work,stable=work/"stable",run=wrong_marker,protect=lambda _:None)
            report={}
            with patch.object(MODULE.COMMON,"library_artifact",return_value="/work/target/debug/libtime_domain.rlib"), \
                 patch.object(MODULE,"copy_contract_dependencies",return_value=dependency):
                with self.assertRaisesRegex(MODULE.Incomplete,"Legal external consumer"):
                    MODULE.contracts(session,{"contracts_wall_seconds":300,"jobs":6},report)
            self.assertNotIn("contracts",report)


if __name__ == "__main__":
    unittest.main()
