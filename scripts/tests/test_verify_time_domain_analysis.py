"""Independent exact graph/seed/feature controls; no compiler or package code runs."""
from __future__ import annotations

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


class DomainAnalysisTests(unittest.TestCase):
    """Valid neighbors discriminate missing graph checks and meaningless seeds."""

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
            if item["name"] in product:
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
        self.assertEqual(len(healthy["critical_files"]), 5)
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


if __name__ == "__main__":
    unittest.main()
