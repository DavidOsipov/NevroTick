"""Independent finite evidence-accounting and artifact-custody controls."""
from __future__ import annotations

from contextlib import contextmanager
import copy
import difflib
import importlib.util
import hashlib
import json
import os
from pathlib import Path
import re
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("portable_analysis_test", ROOT / "scripts/portable-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
FILES = tuple((name, f"crates/time-domain/src/{name}.rs")
              for name in ("coordinate", "era", "exchange", "clock", "rounding"))
SCOPE = MODULE.Scope("time-domain", FILES, "policy/time-domain-analysis.json")
CORE_FILES = (
    ("regression", "crates/time-core/src/regression.rs"),
    ("residuals", "crates/time-core/src/regression/residuals.rs"),
    ("normalization", "crates/time-core/src/regression/normalization.rs"),
    ("history", "crates/time-core/src/regression/history.rs"),
    ("runs", "crates/time-core/src/regression/runs.rs"),
    ("selection", "crates/time-core/src/selection.rs"),
)


def phase_records(summary="Success"):
    """Pinned literal Cargo phase schema, independent of the parser under test."""
    build = {"phase": "Build", "duration": 0.25, "process_status": "Success",
             "argv": ["cargo", "test", "--no-run", "--verbose", "--package=time-domain@0.0.0", "--frozen"]}
    test = {"phase": "Test", "duration": 0.5, "process_status": "Success",
            "argv": ["cargo", "test", "--verbose", "--package=time-domain@0.0.0", "--frozen"]}
    if summary == "Unviable":
        return [{**build, "process_status": {"Failure": 101}}]
    if summary == "CaughtMutant":
        test["process_status"] = {"Failure": 101}
    return [build, test]


def complete_report(value):
    """Complete older minimal fixtures with independently specified native facts."""
    value = json.loads(json.dumps(value))
    for outcome in value["outcomes"]:
        if not outcome.get("phase_results") or outcome["phase_results"] in ([{"phase": "Test"}], [{"phase": "Build"}]):
            outcome["phase_results"] = phase_records(outcome["summary"])
    summaries = [item["summary"] for item in value["outcomes"] if item.get("scenario") != "Baseline"]
    value.update(caught=summaries.count("CaughtMutant"), missed=summaries.count("MissedMutant"),
                 timeout=summaries.count("Timeout"), unviable=summaries.count("Unviable"), success=0)
    return value


class CargoContractArtifactTests(unittest.TestCase):
    """Synthetic layout controls; actual compiler messages are separate evidence."""

    def fixture(self, root, *, layout="new"):
        work = root / "work"; source = work / "source"; vendor = root / "vendor"
        (source / "crates/time-domain").mkdir(parents=True); vendor.mkdir()
        (source / "Cargo.lock").write_text('version=4\n[[package]]\nname="time-domain"\nversion="0.0.0"\n'
            '[[package]]\nname="other-local-owner"\nversion="0.0.0"\n'
            '[[package]]\nname="garde"\nversion="0.23.0"\nsource="registry+https://github.com/rust-lang/crates.io-index"\n')
        (source / "crates/time-domain/Cargo.toml").write_text('[package]\nname="time-domain"\nversion="0.0.0"\n')
        (vendor / "garde-0.23.0").mkdir()
        (vendor / "garde-0.23.0/Cargo.toml").write_text('[package]\nname="garde"\nversion="0.23.0"\n')
        records = []
        for package, manifest, identifier in (
            ("time-domain", "/work/source/crates/time-domain/Cargo.toml", "path+file:///work/source/crates/time-domain#0.0.0"),
            ("garde", str(vendor / "garde-0.23.0/Cargo.toml"), "registry+https://github.com/rust-lang/crates.io-index#garde@0.23.0")):
            folder = "/work/target/debug/deps" if layout == "old" else "/work/target/debug/build/" + package + "/" + "1" * 16 + "/out"
            files = [folder + "/lib" + package.replace("-", "_") + "-" + "2" * 16 + suffix for suffix in (".rlib", ".rmeta")]
            records.append({"reason": "compiler-artifact", "package_id": identifier, "manifest_path": manifest,
                "target": {"name": package.replace("-", "_"), "kind": ["lib"], "crate_types": ["lib"],
                           "src_path": str(Path(manifest).parent / "src/lib.rs")},
                "profile": {"test": False, "opt_level": "0", "overflow_checks": True, "debug_assertions": True},
                "filenames": files})
            for filename in files:
                local = work / filename.removeprefix("/work/"); local.parent.mkdir(parents=True, exist_ok=True)
                local.write_bytes(b"synthetic " + filename.encode())
        session = SimpleNamespace(work=work, source=source, vendor=vendor, protect=lambda path: None)
        return session, records

    def test_new_and_legacy_layout_copy_only_attributed_complete_units(self):
        for layout in ("old", "new"):
            with self.subTest(layout=layout), tempfile.TemporaryDirectory() as temporary:
                session, records = self.fixture(Path(temporary), layout=layout)
                emitted = records[0]["filenames"][0]
                rogue = session.work / "target/debug/deps/libunattributed.rlib"
                rogue.parent.mkdir(parents=True, exist_ok=True); rogue.write_bytes(b"not attributed")
                island = MODULE.copy_cargo_contract_dependencies(session, "\n".join(map(json.dumps, records)),
                    {"libtime_domain.rlib": emitted})
                self.assertNotIn(rogue.name, {item.name for item in island.iterdir()})
                copied = island / "libtime_domain.rlib"
                original = session.work / emitted.removeprefix("/work/")
                self.assertEqual(copied.read_bytes(), original.read_bytes())
                self.assertNotEqual(copied.stat().st_ino, original.stat().st_ino)
                self.assertEqual(copied.stat().st_nlink, 1)
                self.assertEqual(copied.stat().st_mode & 0o777, 0o400)

    def test_wrong_identity_role_profile_inventory_and_paths_are_rejected(self):
        for fault in ("package", "manifest", "local_absolute", "kind", "target", "profile", "test", "missing", "duplicate", "escape", "symlink", "wrong_unit"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                session, records = self.fixture(Path(temporary))
                emitted = records[0]["filenames"][0]
                row = records[1]
                if fault == "package": row["package_id"] += "-wrong"
                if fault == "manifest": row["manifest_path"] = "/unowned/Cargo.toml"
                if fault == "local_absolute": records[0]["manifest_path"] = "/work/source//tmp/other/Cargo.toml"
                if fault == "kind": row["target"]["kind"] = ["proc-macro"]
                if fault == "target": row["filenames"] = [value.replace("/debug/", "/x86_64-pc-windows-msvc/debug/") for value in row["filenames"]]
                if fault == "profile": row["profile"]["opt_level"] = "3"
                if fault == "test": row["profile"]["test"] = True
                if fault == "missing": row["filenames"].pop()
                if fault == "duplicate": records.append(copy.deepcopy(row))
                if fault == "escape": row["filenames"][0] = "/work/target/debug/../outside.rlib"
                if fault == "wrong_unit": row["filenames"] = [value.replace("/garde/", "/time-domain/") for value in row["filenames"]]
                if fault == "symlink":
                    local = session.work / row["filenames"][0].removeprefix("/work/")
                    local.unlink(); local.symlink_to(session.work / emitted.removeprefix("/work/"))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.copy_cargo_contract_dependencies(session, "\n".join(map(json.dumps, records)), {"libtime_domain.rlib": emitted})

    def test_explicit_library_alias_cannot_substitute_another_crate_or_metadata(self):
        for fault in ("wrong_crate", "metadata"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                session, records = self.fixture(Path(temporary))
                substituted = records[1]["filenames"][0] if fault == "wrong_crate" else records[0]["filenames"][1]
                with self.assertRaisesRegex(MODULE.Incomplete, "Required external-contract library"):
                    MODULE.copy_cargo_contract_dependencies(session, "\n".join(map(json.dumps, records)),
                                                           {"libtime_domain.rlib": substituted})


class AnalyzerOutputTests(unittest.TestCase):
    """Reach actual lane consumers with literal healthy analyzer-shaped output."""

    def run_fixture(self, lane, change=lambda _path, _kind: None, *, report=None):
        """Fake only the custodian; execute actual report and denominator checks."""
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            work = base / "work"; work.mkdir()
            subjects = {"era": dict(FILES)["era"]}
            mutant = {"file": subjects["era"], "name": "one", "package": "time-domain"}
            calls = []
            report = {} if report is None else report

            def run(label, _command, _timeout, **_options):
                calls.append(label)
                if label == "coverage":
                    path = work / "coverage.json"
                    path.write_text(json.dumps({"data": [{"files": [{
                        "filename": "/work/source/" + subjects["era"],
                        "summary": {"lines": {"count": 10, "covered": 9}}}]}]}))
                    change(path, "coverage")
                    return "test result: ok. 3 passed; 0 failed; 0 ignored;"
                if label == "mutation-enumeration:era":
                    return json.dumps([mutant])
                self.assertEqual(label, "mutation-campaign:era")
                directory = work / "mutation/era/mutants.out"
                (directory / "log").mkdir(parents=True)
                log = directory / "log/baseline.log"
                log.write_text("test result: ok. 3 passed; 0 failed; 0 ignored;")
                change(log, "baseline")
                path = directory / "outcomes.json"
                path.write_text(json.dumps(complete_report({
                    "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})))
                change(path, "outcomes")
                return {"exit_code": 0, "failure": None, "output": "complete"}

            MODULE.run_analysis_lanes(SCOPE, subjects,
                {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
                 "maximum_mutants": 512, "mutation_jobs": 2},
                work, base / "tools", run, report, lane)
            return report, calls

    def test_healthy_actual_lanes_keep_independent_counts(self):
        coverage, calls = self.run_fixture("coverage")
        self.assertEqual(calls, ["coverage"])
        self.assertEqual(coverage["coverage"]["era"]["line_percent"], 90)
        self.assertEqual(coverage["coverage"]["era"]["tests_executed"], 3)
        mutation, calls = self.run_fixture("mutation")
        self.assertEqual(calls, ["mutation-enumeration:era", "mutation-campaign:era"])
        self.assertEqual(mutation["mutation"]["era"]["viable"], 1)
        self.assertEqual(mutation["mutation"]["era"]["baseline_tests_executed"], 3)
        self.assertTrue(mutation["mutation"]["era"]["gate_passed"])

    def test_actual_lanes_reject_linked_json_outputs(self):
        for kind, lane in (("coverage", "coverage"), ("outcomes", "mutation"), ("baseline", "mutation")):
            def link(path, selected):
                if selected == kind:
                    target = path.with_name("retained-" + path.name)
                    path.rename(target)
                    path.symlink_to(target)
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                self.run_fixture(lane, link)

    def test_actual_lanes_reject_linked_parent_directories(self):
        for kind, lane in (("coverage", "coverage"), ("outcomes", "mutation"), ("baseline", "mutation")):
            def link(path, selected):
                if selected == kind:
                    parent = path.parent
                    target = parent.with_name(parent.name + "-retained")
                    parent.rename(target)
                    parent.symlink_to(target, target_is_directory=True)
            with self.subTest(kind=kind), self.assertRaises(MODULE.Incomplete):
                self.run_fixture(lane, link)

    def test_actual_lanes_reject_special_and_oversized_outputs_without_reading(self):
        for kind, lane in (("coverage", "coverage"), ("outcomes", "mutation"), ("baseline", "mutation")):
            for defect in ("fifo", "directory", "oversize", "hardlink"):
                def change(path, selected):
                    if selected != kind:
                        return
                    if defect == "oversize":
                        bound = (MODULE.ANALYSIS_LOG_MAX_BYTES if kind == "baseline"
                                 else MODULE.ANALYSIS_JSON_MAX_BYTES)
                        with path.open("ab") as stream:
                            stream.truncate(bound + 1)  # Sparse: no large payload allocation/write.
                    elif defect == "hardlink":
                        os.link(path, path.with_name("second-link"))
                    else:
                        path.unlink()
                        if defect == "fifo":
                            os.mkfifo(path)
                        else:
                            path.mkdir()
                with self.subTest(kind=kind, defect=defect), self.assertRaises(MODULE.Incomplete):
                    self.run_fixture(lane, change)

    def test_actual_json_consumers_reject_malformed_bounded_payloads(self):
        invalid = {
            "duplicate": '{"data":[],"data":[]}',
            "escaped-duplicate": '{"data":[],"d\\u0061ta":[]}',
            "nonfinite": '{"unused":NaN}',
            "infinity": '{"unused":Infinity}',
            "float-overflow": '{"unused":1e999}',
            "overlong-number": '{"unused":' + "1" * 129 + '}',
            "depth": '{"unused":' + "[" * 64 + "0" + "]" * 64 + '}',
            "utf8": b'{"unused":"\xff"}',
            "trailing": '{} {}',
            "truncated": '{"data":',
            "root-array": '[]',
        }
        for kind, lane in (("coverage", "coverage"), ("outcomes", "mutation")):
            for defect, raw in invalid.items():
                def change(path, selected):
                    if selected == kind:
                        path.write_bytes(raw if isinstance(raw, bytes) else raw.encode())
                report = {}
                with self.subTest(kind=kind, defect=defect), self.assertRaises(MODULE.Incomplete):
                    self.run_fixture(lane, change, report=report)
                self.assertNotIn("era", report.get(lane, {}))

    def test_healthy_coverage_larger_than_one_mib_retains_actual_counts(self):
        """Retained LLVM reports exceed 4 MiB; a 1 MiB replacement cap is invalid."""
        def pad(path, kind):
            if kind == "coverage":
                raw = path.read_bytes()
                path.write_bytes(b" " * (1024 * 1024 + 1) + raw)
        report, _calls = self.run_fixture("coverage", pad)
        self.assertEqual(report["coverage"]["era"]["line_percent"], 90)
        self.assertEqual(report["coverage"]["era"]["tests_executed"], 3)

    def test_bounded_read_rejects_growth_and_leaf_or_parent_replacement(self):
        for defect in ("growth-within-bound", "growth-past-bound", "leaf", "parent"):
            with self.subTest(defect=defect), tempfile.TemporaryDirectory() as temporary:
                base = Path(temporary); parent = base / "output"; parent.mkdir()
                path = parent / "report"; path.write_bytes(b"healthy")
                actual_read, requests = os.read, []

                def raced_read(descriptor, requested):
                    requests.append(requested)
                    if len(requests) == 1:
                        if defect.startswith("growth"):
                            with path.open("ab") as stream:
                                stream.write(b"more" if defect == "growth-within-bound" else b"x" * 64)
                        elif defect == "leaf":
                            path.rename(parent / "detached")
                            path.write_bytes(b"healthy")
                        else:
                            parent.rename(base / "detached")
                            parent.mkdir()
                            path.write_bytes(b"healthy")
                    return actual_read(descriptor, requested)

                with patch.object(MODULE.os, "read", side_effect=raced_read):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.read_analysis_bytes(path, maximum=16)
                self.assertTrue(requests)
                self.assertTrue(all(0 < request <= 17 for request in requests))

    def test_bounded_reader_rejects_special_files_before_read_and_wrong_owner(self):
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            fifo = parent / "fifo"; os.mkfifo(fifo)
            for path in (fifo, parent, parent / "missing"):
                with self.subTest(path=path.name), patch.object(MODULE.os, "read") as read:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.read_analysis_bytes(path, maximum=16)
                    read.assert_not_called()
            path = parent / "owned"; path.write_bytes(b"valid")
            with patch.object(MODULE.os, "geteuid", return_value=os.geteuid() + 1):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_analysis_bytes(path, maximum=16)
            path.write_bytes(b"x" * 17)
            with patch.object(MODULE.os, "read") as read:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_analysis_bytes(path, maximum=16)
                read.assert_not_called()
            for bound in (True, 0, -1, MODULE.ANALYSIS_JSON_MAX_BYTES + 1):
                with self.subTest(bound=bound), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_analysis_bytes(path, maximum=bound)

    def test_path_limits_and_invalid_encoding_reject_before_any_open(self):
        cases = (
            Path("/" + "x" * 4096),
            Path("/" + "é" * 2048),
            Path("/" + "/".join(["x"] * 65)),
            Path("/invalid-\ud800"),
        )
        for index, path in enumerate(cases):
            with self.subTest(case=index), patch.object(MODULE.os, "open", side_effect=OSError("unexpected open")) as opened:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_analysis_bytes(path, maximum=16)
                opened.assert_not_called()

    def test_exact_path_limits_keep_healthy_read_and_one_below_never_opens(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "healthy-é"; path.write_bytes(b"healthy")
            limits = {"ANALYSIS_PATH_MAX_BYTES": len(str(path).encode("utf-8")),
                      "ANALYSIS_PATH_MAX_COMPONENTS": len(path.parts) - 1}
            with patch.multiple(MODULE, **limits):
                self.assertEqual(MODULE.read_analysis_bytes(path, maximum=16), b"healthy")
            for selected in limits:
                tighter = {**limits, selected: limits[selected] - 1}
                with self.subTest(limit=selected), patch.multiple(MODULE, **tighter):
                    with patch.object(MODULE.os, "open", side_effect=OSError("unexpected open")) as opened:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.read_analysis_bytes(path, maximum=16)
                        opened.assert_not_called()

    def test_raw_path_bounds_precede_path_construction_and_file_open(self):
        for raw in ("x" * 4097, "x/" * 2049, "é" * 2049, "invalid-\ud800"):
            with self.subTest(raw_length=len(raw)), patch.object(MODULE, "Path") as constructor:
                with patch.object(MODULE.os, "open", side_effect=OSError("unexpected open")) as opened:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.analysis_output_path(raw)
                    constructor.assert_not_called()
                    opened.assert_not_called()
        self.assertEqual(MODULE.analysis_output_path("x" * 4096), Path("x" * 4096))
        self.assertEqual(MODULE.analysis_output_path("é" * 2048), Path("é" * 2048))
        self.assertEqual(MODULE.analysis_output_path("/".join(["x"] * 64)), Path(*["x"] * 64))
        for raw in ("/".join(["x"] * 65), None, False, []):
            with self.subTest(kind=type(raw).__name__), self.assertRaises(MODULE.Incomplete):
                MODULE.analysis_output_path(raw)

    def test_json_work_and_depth_are_bounded_before_decode(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "report.json"
            for raw, limits in (
                    ('{"many":[0,0,0]}', {"ANALYSIS_JSON_MAX_MARKERS": 5}),
                    ('{"deep":[[0]]}', {"ANALYSIS_JSON_MAX_DEPTH": 2})):
                path.write_text(raw)
                with patch.multiple(MODULE, **limits), patch.object(MODULE.json, "loads") as decode:
                    with self.assertRaisesRegex(MODULE.Incomplete, "structural work or depth"):
                        MODULE.read_analysis_json(path)
                    decode.assert_not_called()
            path.write_text('{"literal":"[{},:\\"\\\\]", "number":1.25, "count":12}')
            self.assertEqual(MODULE.read_analysis_json(path),
                             {"literal": '[{},:"\\]', "number": 1.25, "count": 12})

    def test_baseline_invalid_utf8_and_empty_relative_path_remain_incomplete(self):
        def invalid_utf8(path, kind):
            if kind == "baseline":
                path.write_bytes(b"\xff")
        with self.assertRaises(MODULE.Incomplete):
            self.run_fixture("mutation", invalid_utf8)

        def empty_log(path, kind):
            if kind == "outcomes":
                path.write_text(path.read_text().replace('"log/baseline.log"', '""'))
        with self.assertRaises(MODULE.Incomplete):
            self.run_fixture("mutation", empty_log)

    def test_actual_baseline_rejects_oversized_raw_path_before_splitting(self):
        def oversized_log(path, kind):
            if kind == "outcomes":
                path.write_text(path.read_text().replace('"log/baseline.log"', json.dumps("x/" * 2049)))
        with self.assertRaisesRegex(MODULE.Incomplete, "path exceeds"):
            self.run_fixture("mutation", oversized_log)


class EvidenceTests(unittest.TestCase):
    """Literal healthy controls accompany wrong-file/empty/cause failures."""

    @contextmanager
    def _session_admission_fixture(self, free_bytes, *, drop_during_admission=False):
        """Reach the real initializer with bounded fake expensive admissions.

        Real temporary checkout/vendor/output-parent paths preserve the actual
        path checks. A sentinel stops at the first output mkdir before copying,
        tool execution or product work; these are runner-order controls only.
        """
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary).resolve()
            root = parent / "checkout"; root.mkdir()
            vendor = parent / "vendor"; vendor.mkdir()
            output = parent / "output"
            args = SimpleNamespace(vendor_dir=vendor, output_dir=output, lane="contracts")
            events, measured_free = [], []
            current_free = [free_bytes]

            class OutputReached(RuntimeError):
                """Healthy initialization reached its first owned output mutation."""

            def product_admission(actual_root):
                self.assertEqual(actual_root, root)
                events.append("product")
                return {}, {}

            def equivalence_admission(actual_root, scope, policy):
                self.assertEqual((actual_root, scope, policy), (root, SCOPE, {}))
                events.append("equivalences")
                return []

            def input_hash(path):
                self.assertTrue(path.is_relative_to(root))
                events.append("hash")
                return "a" * 64

            def analysis_admission(actual_root):
                self.assertEqual(actual_root, root)
                events.append("analysis")
                if drop_during_admission:
                    current_free[0] = 2 * 1024 ** 3 - 1
                return {"base": parent / "admitted-base", "stable": parent / "stable"}

            def vendor_admission(actual_vendor, policy):
                self.assertEqual((actual_vendor, policy), (vendor, {}))
                events.append("vendor")
                return {}

            def disk_usage(path):
                self.assertEqual(path, parent)
                events.append("space")
                measured_free.append(current_free[0])
                return SimpleNamespace(free=current_free[0])

            def first_output_mkdir(path, *positional, **keywords):
                self.assertEqual(path, output)
                self.assertEqual((positional, keywords), ((), {"mode": 0o700}))
                events.append("output-mkdir")
                raise OutputReached("first private output mkdir reached")

            with (patch.object(MODULE.PRODUCT, "admit_product", side_effect=product_admission) as product,
                  patch.object(MODULE, "admit_mutation_equivalences", side_effect=equivalence_admission) as proofs,
                  patch.object(MODULE.PRODUCT, "sha256", side_effect=input_hash) as hashes,
                  patch.object(MODULE.ANALYSIS, "admit", side_effect=analysis_admission) as analysis,
                  patch.object(MODULE.PRODUCT, "admit_vendor", side_effect=vendor_admission) as admitted_vendor,
                  patch.object(MODULE.shutil, "disk_usage", side_effect=disk_usage) as space,
                  patch.object(Path, "mkdir", side_effect=first_output_mkdir, autospec=True) as mkdir):
                yield SimpleNamespace(
                    root=root, args=args, report={}, output=output, events=events,
                    measured_free=measured_free, output_reached=OutputReached,
                    product=product, proofs=proofs, hashes=hashes, analysis=analysis,
                    vendor=admitted_vendor, space=space, mkdir=mkdir,
                )

    def test_session_low_space_rejects_before_expensive_admission(self):
        """One byte below the unchanged floor must avoid all expensive admission."""
        with self._session_admission_fixture(2 * 1024 ** 3 - 1) as case:
            with self.assertRaisesRegex(MODULE.Incomplete, "at least 2 GiB"):
                MODULE.Session(case.root, case.args, case.report, SCOPE, {}, ())
            self.assertEqual(
                case.events, ["space"],
                "low space must reject before product, proof, hash, tool or vendor admission",
            )
            for expensive in (case.product, case.proofs, case.hashes, case.analysis, case.vendor):
                expensive.assert_not_called()
            case.mkdir.assert_not_called()
            self.assertFalse(case.output.exists())
            self.assertEqual(case.report, {})

    def test_session_healthy_space_preserves_mandatory_admission_before_output(self):
        """The exact floor permits mandatory admission before the first mkdir."""
        with self._session_admission_fixture(2 * 1024 ** 3) as case:
            with self.assertRaisesRegex(case.output_reached, "first private output mkdir"):
                MODULE.Session(case.root, case.args, case.report, SCOPE, {}, ())
            admitted = [event for event in case.events if event not in {"space", "output-mkdir"}]
            self.assertEqual(admitted, ["product", "equivalences", *["hash"] * 7, "analysis", "vendor"])
            for mandatory in (case.product, case.proofs, case.analysis, case.vendor):
                self.assertEqual(mandatory.call_count, 1)
            self.assertEqual(case.hashes.call_count, 7)
            last_space = len(case.events) - 1 - case.events[::-1].index("space")
            self.assertLess(case.events.index("vendor"), last_space)
            self.assertLess(last_space, case.events.index("output-mkdir"))
            self.assertTrue(all(value == 2 * 1024 ** 3 for value in case.measured_free))
            case.mkdir.assert_called_once()
            self.assertFalse(case.output.exists())
            self.assertEqual(case.report, {})

    def test_session_space_drop_during_admission_rejects_before_output(self):
        """A fresh late guard still rejects space lost during mandatory admission."""
        with self._session_admission_fixture(2 * 1024 ** 3, drop_during_admission=True) as case:
            with self.assertRaisesRegex(MODULE.Incomplete, "at least 2 GiB"):
                MODULE.Session(case.root, case.args, case.report, SCOPE, {}, ())
            admitted = [event for event in case.events if event != "space"]
            self.assertEqual(admitted, ["product", "equivalences", *["hash"] * 7, "analysis", "vendor"])
            for mandatory in (case.product, case.proofs, case.analysis, case.vendor):
                self.assertEqual(mandatory.call_count, 1)
            self.assertEqual(case.hashes.call_count, 7)
            self.assertEqual(case.measured_free[-1], 2 * 1024 ** 3 - 1)
            self.assertEqual(case.events[-1], "space")
            self.assertLess(case.events.index("vendor"), len(case.events) - 1)
            case.mkdir.assert_not_called()
            self.assertFalse(case.output.exists())
            self.assertEqual(case.report, {})

    def test_stable_core_profile_requires_explicit_exact_policy_scope_and_lane(self):
        core = MODULE.Scope("time-core", CORE_FILES, "policy/time-core-analysis.json")
        self.assertEqual(MODULE.admission_profile(SimpleNamespace(lane="contracts"), SCOPE, {}), "full")
        policy = {"tool_admission_profile": "stable-core"}
        for lane in ("contracts", "coverage", "mutation"):
            args = SimpleNamespace(lane=lane, tool_admission_profile="stable-core")
            self.assertEqual(MODULE.admission_profile(args, core, policy), "stable-core")
        for lane, profile, scope, selected_policy in (
                ("contracts", "stable-core", core, {}),
                ("contracts", "full", core, policy),
                ("contracts", "stable-core", SCOPE, policy),
                ("contracts", "stable-core", MODULE.Scope("time-core", CORE_FILES, "other.json"), policy),
                *((lane, "stable-core", core, policy) for lane in ("fuzz", "all", "inventory", "cohort-union")),
                *(("contracts", value, core, {}) for value in (None, True, "", "stable", "unknown"))):
            with self.subTest(lane=lane, profile=profile, scope=scope), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile=profile),
                                         scope, selected_policy)

    @staticmethod
    def service_lifecycle_role():
        """Independent service-only authority card; no product or tool operations."""
        subjects = (("lifecycle", "crates/windows-platform/src/service.rs"),)
        scope = MODULE.Scope("windows-platform", subjects, "policy/service-analysis.json")
        policy = {
            "schema_version": 1, "package": "windows-platform", "subjects": dict(subjects),
            "critical_files": ["crates/windows-platform/src/service.rs"], "tool_admission_profile": "stable-core",
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "coverage_wall_seconds": 300, "enumeration_wall_seconds": 120,
            "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "jobs": 6, "test_threads": 1, "coverage_test_threads": 1, "mutation_jobs": 4,
            "mutation_process_limit": 128, "mutation_message_format": "json", "maximum_mutants": 512,
            "mutation_equivalences": [], "binding": None, "mutation_inventories": {},
        }
        return scope, policy

    def test_service_profile_uses_stable_payload_only_for_its_exact_lanes(self):
        scope, policy = self.service_lifecycle_role()
        with patch.object(MODULE.ANALYSIS, "admit") as tools:
            for lane in ("coverage", "mutation", "all"):
                self.assertEqual(MODULE.admission_profile(
                    SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy), "stable-core")
            tools.assert_not_called()
        for lane in ("enumerate", "contracts", "fuzz", "proof", "inventory"):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane="coverage", tool_admission_profile="full"), scope,
                                     {**policy, "tool_admission_profile": "full"})

    def test_service_profile_cannot_borrow_another_package_file_or_policy(self):
        scope, policy = self.service_lifecycle_role()
        args = SimpleNamespace(lane="coverage", tool_admission_profile="stable-core")
        for changed in (MODULE.Scope("time-core", scope.subjects, scope.policy),
                        MODULE.Scope(scope.package, (("other", scope.subjects[0][1]),), scope.policy),
                        MODULE.Scope(scope.package, (("lifecycle", "crates/windows-platform/src/other.rs"),), scope.policy),
                        MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json"),
                        MODULE.Scope(scope.package, scope.subjects * 2, scope.policy)):
            with self.subTest(scope=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, changed, policy)

    def test_service_profile_rejects_weakened_limits_exceptions_and_missing_fields(self):
        scope, policy = self.service_lifecycle_role()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        for key, value in (("schema_version", True), ("package", "time-core"),
                ("critical_files", []), ("subjects", {}), ("coverage_minimum_line_percent", 89),
                ("mutation_minimum_viable_killed_percent", 94), ("mutation_jobs", 12),
                ("mutation_process_limit", 512), ("jobs", 12), ("test_threads", 6),
                ("coverage_test_threads", 6), ("coverage_wall_seconds", 600),
                ("enumeration_wall_seconds", 300), ("mutation_wall_seconds", 3600),
                ("mutation_build_seconds", 120), ("mutation_test_seconds", 90),
                ("maximum_mutants", 1024), ("mutation_message_format", "human"),
                ("mutation_equivalences", [{"excluded": True}]), ("mutation_error_values", {})):
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {field: value for field, value in policy.items() if field != key})

    def test_service_schema2_requires_both_exact_portable_owners_and_keeps_schema1(self):
        old_scope, old_policy = self.service_lifecycle_role()
        subjects = (('lifecycle', 'crates/windows-platform/src/service.rs'),
                    ('scm', 'crates/windows-platform/src/scm.rs'))
        scope = MODULE.Scope('windows-platform', subjects, 'policy/service-analysis.json')
        policy = {**old_policy, 'schema_version': 2, 'subjects': dict(subjects),
                  'critical_files': [path for _, path in subjects]}
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        self.assertEqual(MODULE.admission_profile(args, scope, policy), 'stable-core')
        self.assertEqual(MODULE.admission_profile(args, old_scope, old_policy), 'stable-core')
        for changed_scope in (old_scope, MODULE.Scope(scope.package, subjects[1:], scope.policy),
                              MODULE.Scope(scope.package, tuple(reversed(subjects)), scope.policy),
                              MODULE.Scope(scope.package, (*subjects, ('native', 'crates/windows-platform/src/native.rs')), scope.policy)):
            with self.subTest(scope=changed_scope), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, changed_scope, policy)
        for change in ({'schema_version': 1}, {'schema_version': 3},
                       {'subjects': dict(subjects[:1])}, {'critical_files': [subjects[0][1]]},
                       {'mutation_minimum_viable_killed_percent': 94}, {'mutation_jobs': 12},
                       {'mutation_error_values': {}}, {'mutation_equivalences': [{'excluded': True}]}):
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, **change})

    def test_service_schema3_reduces_workers_without_widening_limits_or_omitting_owners(self):
        old_scope, old_policy = self.service_lifecycle_role()
        subjects = (('lifecycle', 'crates/windows-platform/src/service.rs'),
                    ('scm', 'crates/windows-platform/src/scm.rs'))
        scope = MODULE.Scope('windows-platform', subjects, 'policy/service-analysis.json')
        schema2 = {**old_policy, 'schema_version': 2, 'subjects': dict(subjects),
                   'critical_files': [path for _, path in subjects]}
        policy = {**schema2, 'schema_version': 3, 'mutation_jobs': 1}
        for lane in ('coverage', 'mutation', 'all'):
            args = SimpleNamespace(lane=lane, tool_admission_profile='stable-core')
            self.assertEqual(MODULE.admission_profile(args, scope, policy), 'stable-core')
            self.assertEqual(MODULE.admission_profile(args, scope, schema2), 'stable-core')
            self.assertEqual(MODULE.admission_profile(args, old_scope, old_policy), 'stable-core')
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for change in ({'mutation_jobs': 4}, {'mutation_jobs': 2}, {'mutation_jobs': True},
                       {'mutation_process_limit': 512}, {'jobs': 12}, {'mutation_wall_seconds': 3600},
                       {'mutation_minimum_viable_killed_percent': 94}, {'coverage_minimum_line_percent': 89},
                       {'subjects': dict(subjects[:1])}, {'critical_files': [subjects[0][1]]}):
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, **change})
        for wrong in (old_scope, MODULE.Scope(scope.package, subjects[1:], scope.policy)):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)

    def test_service_schema4_parallel_role_preserves_all_historical_rows(self):
        old_scope, old_policy = self.service_lifecycle_role()
        subjects = (('lifecycle', 'crates/windows-platform/src/service.rs'),
                    ('scm', 'crates/windows-platform/src/scm.rs'))
        scope = MODULE.Scope('windows-platform', subjects, 'policy/service-analysis.json')
        for schema, jobs, limit in ((1, 4, 128), (2, 4, 128), (3, 1, 128), (4, 4, 512)):
            selected = old_scope if schema == 1 else scope
            policy = {**old_policy, 'schema_version': schema, 'subjects': dict(selected.subjects),
                      'critical_files': [path for _, path in selected.subjects],
                      'mutation_jobs': jobs, 'mutation_process_limit': limit}
            for lane in ('coverage', 'mutation', 'all'):
                with self.subTest(schema=schema, lane=lane):
                    self.assertEqual(MODULE.admission_profile(
                        SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), selected, policy),
                        'stable-core')

    def test_service_schema5_adds_only_the_exact_privilege_owner(self):
        _, historical = self.service_lifecycle_role()
        subjects = (('lifecycle', 'crates/windows-platform/src/service.rs'),
                    ('scm', 'crates/windows-platform/src/scm.rs'),
                    ('privilege', 'crates/windows-platform/src/privilege_observation.rs'))
        scope = MODULE.Scope('windows-platform', subjects, 'policy/service-analysis.json')
        policy = {**historical, 'schema_version': 5, 'subjects': dict(subjects),
                  'critical_files': [path for _, path in subjects], 'mutation_jobs': 4,
                  'mutation_process_limit': 512}
        for lane in ('coverage', 'mutation', 'all'):
            self.assertEqual(MODULE.admission_profile(
                SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for wrong in (MODULE.Scope(scope.package, subjects[:2], scope.policy),
                      MODULE.Scope(scope.package, subjects[2:], scope.policy),
                      MODULE.Scope(scope.package, tuple(reversed(subjects)), scope.policy),
                      MODULE.Scope(scope.package, (*subjects, ('native', 'native.rs')), scope.policy)):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)
        for key, value in (('schema_version', 4), ('schema_version', 6), ('schema_version', True),
                           ('schema_version', 5.0), ('mutation_jobs', 1), ('mutation_jobs', 8),
                           ('mutation_jobs', True), ('mutation_process_limit', 128),
                           ('subjects', dict(subjects[:2])), ('critical_files', [path for _, path in subjects[:2]]),
                           ('coverage_minimum_line_percent', 89), ('mutation_minimum_viable_killed_percent', 94),
                           ('maximum_mutants', 513), ('mutation_equivalences', [{'excluded': True}])):
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for lane in ('enumerate', 'contracts', 'fuzz', 'proof'):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)

    def test_service_schemas_reject_crossed_worker_and_task_profiles(self):
        old_scope, old_policy = self.service_lifecycle_role()
        subjects = (('lifecycle', 'crates/windows-platform/src/service.rs'),
                    ('scm', 'crates/windows-platform/src/scm.rs'))
        scope = MODULE.Scope('windows-platform', subjects, 'policy/service-analysis.json')
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for schema, jobs, limit in ((1, 4, 128), (2, 4, 128), (3, 1, 128), (4, 4, 512)):
            selected = old_scope if schema == 1 else scope
            policy = {**old_policy, 'schema_version': schema, 'subjects': dict(selected.subjects),
                      'critical_files': [path for _, path in selected.subjects],
                      'mutation_jobs': jobs, 'mutation_process_limit': limit}
            for wrong_jobs, wrong_limit in ((1, 128), (4, 128), (4, 512), (8, 512), (12, 512), (4, 256)):
                if (wrong_jobs, wrong_limit) == (jobs, limit):
                    continue
                with self.subTest(schema=schema, jobs=wrong_jobs, limit=wrong_limit), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, selected, {**policy, 'mutation_jobs': wrong_jobs,
                                                             'mutation_process_limit': wrong_limit})
            for key, value in (('schema_version', True), ('schema_version', float(schema)),
                               ('schema_version', 0), ('schema_version', 5), ('schema_version', '4'),
                               ('mutation_jobs', True), ('mutation_jobs', float(jobs)),
                               ('mutation_process_limit', True), ('mutation_process_limit', float(limit))):
                with self.subTest(schema=schema, key=key, value=value), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, selected, {**policy, key: value})
            if schema == 4:
                for wrong in (old_scope, MODULE.Scope(scope.package, subjects[1:], scope.policy),
                              MODULE.Scope(scope.package, tuple(reversed(subjects)), scope.policy)):
                    with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                        MODULE.admission_profile(args, wrong, policy)
                for lane in ('enumerate', 'contracts', 'fuzz', 'proof'):
                    with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                        MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'),
                                                 selected, policy)
                for key, value in (('jobs', 12), ('test_threads', 4), ('coverage_test_threads', 4),
                                   ('coverage_minimum_line_percent', 89), ('mutation_minimum_viable_killed_percent', 94),
                                   ('mutation_build_seconds', 120), ('mutation_test_seconds', 90),
                                   ('mutation_wall_seconds', 3600), ('tool_admission_profile', 'full'),
                                   ('mutation_error_values', {}), ('mutation_equivalences', [{'excluded': True}])):
                    with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.admission_profile(args, selected, {**policy, key: value})

    @staticmethod
    def tracking_inventory_control():
        """Literal four-owner S152 admission; no source, tool or process effect."""
        subjects = {"projection": "crates/time-core/src/regression/projection.rs",
                    "combination": "crates/time-core/src/combination.rs",
                    "combination_admission": "crates/time-core/src/combination/admission.rs",
                    "combination_lifecycle": "crates/time-core/src/combination/lifecycle.rs"}
        scope = MODULE.Scope("time-core", tuple(subjects.items()),
                             "tasks/s1.3-tracking-inventory-admission.json")
        policy = {"schema_version": 8, "admission_status": "admitted-four-owner-inventory-only",
                  "package": "time-core", "source_file_count": 130, "subjects": subjects,
                  "maximum_mutants": 512, "listing_maximum_per_file": 512,
                  "listing_maximum_total": 2048, "inventory_wall_seconds": 600,
                  "mutation_jobs": 10, "mutation_process_limit": 512,
                  "listing_process_limit": 128, "mutation_message_format": "json",
                  "mutation_equivalences": [], "native_test_invocations": 0,
                  "campaigns_admitted": False, "tool_admission_profile": "stable-core"}
        return scope, policy

    def test_stable_core_inventory_admits_only_the_explicit_tracking_listing_card(self):
        scope, policy = self.tracking_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")

    def test_stable_core_inventory_rejects_changed_profile_scope_and_card_facts(self):
        scope, policy = self.tracking_inventory_control()
        good_args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        invalid = [
            (SimpleNamespace(lane="mutation"), scope, policy),
            (SimpleNamespace(lane="mutation", tool_admission_profile="full"), scope, policy),
            (SimpleNamespace(lane="inventory", tool_admission_profile="stable-core"), scope, policy),
            (SimpleNamespace(lane="coverage", tool_admission_profile="stable-core"), scope, policy),
            (good_args, MODULE.Scope("time-domain", scope.subjects, scope.policy), policy),
            (good_args, MODULE.Scope("time-core", scope.subjects, "tasks/other.json"), policy),
            (good_args, MODULE.Scope("time-core", (("history", "crates/time-core/src/regression/history.rs"),), scope.policy), policy),
            (good_args, MODULE.Scope("time-core", scope.subjects * 2, scope.policy), policy),
            (good_args, MODULE.Scope("time-core", scope.subjects[:-1], scope.policy), policy),
            (good_args, MODULE.Scope("time-core", tuple(reversed(scope.subjects)), scope.policy), policy),
            (good_args, MODULE.Scope("time-core", scope.subjects,
                                     "tasks/s1.2-standard-errors-inventory-admission.json"), policy),
        ]
        for field, value in (("schema_version", 7), ("schema_version", True),
                             ("package", "time-domain"), ("subjects", {}),
                             ("native_test_invocations", 1), ("native_test_invocations", False),
                             ("campaigns_admitted", True), ("campaigns_admitted", 0),
                             ("tool_admission_profile", "full")):
            invalid.append((good_args, scope, {**policy, field: value}))
        for field in ("tool_admission_profile", "subjects", "native_test_invocations", "campaigns_admitted"):
            invalid.append((good_args, scope, {key: value for key, value in policy.items() if key != field}))
        invalid.append((SimpleNamespace(lane="mutation", tool_admission_profile="full"), scope,
                        {**policy, "tool_admission_profile": "full"}))
        for args, candidate_scope, candidate_policy in invalid:
            with self.subTest(args=args, scope=candidate_scope, policy=candidate_policy), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, candidate_scope, candidate_policy)

    @staticmethod
    def chart_inventory_control():
        """Literal S154 listing role; no Session, product or tool admission."""
        subjects = {
            "chart": "crates/time-core/src/regression/chart.rs",
            "chart_roundoff": "crates/time-core/src/regression/chart/roundoff.rs",
            "chart_enclosure": "crates/time-core/src/chart_enclosure.rs",
            "enclosure": "crates/time-core/src/projection_enclosure.rs",
        }
        scope = MODULE.Scope("time-core", tuple(subjects.items()),
                             "tasks/s1.2-chart-inventory-admission.json")
        policy = {"schema_version": 9,
                  "admission_status": "admitted-four-owner-chart-inventory-only",
                  "tool_admission_profile": "stable-core", "package": "time-core",
                  "source_file_count": 136, "subjects": subjects,
                  "maximum_mutants": 512, "listing_maximum_per_file": 512,
                  "listing_maximum_total": 2048, "inventory_wall_seconds": 600,
                  "mutation_jobs": 10, "mutation_process_limit": 512,
                  "listing_process_limit": 128, "mutation_message_format": "json",
                  "mutation_equivalences": [], "native_test_invocations": 0,
                  "campaigns_admitted": False}
        return scope, policy

    def test_stable_core_chart_inventory_admits_only_its_exact_listing_role(self):
        scope, policy = self.chart_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        with patch.object(MODULE, "Session") as session, \
             patch.object(MODULE.ANALYSIS, "admit") as admission:
            self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
            session.assert_not_called()
            admission.assert_not_called()

    def test_stable_core_chart_inventory_rejects_role_and_typed_fact_changes(self):
        scope, policy = self.chart_inventory_control()
        good = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        tracking_scope, tracking_policy = self.tracking_inventory_control()
        invalid = [
            (SimpleNamespace(lane="mutation"), scope, policy),
            (SimpleNamespace(lane="mutation"), scope, {**policy, "tool_admission_profile": "full"}),
            (SimpleNamespace(lane="mutation", tool_admission_profile="full"), scope,
             {**policy, "tool_admission_profile": "full"}),
            (good, MODULE.Scope("time-domain", scope.subjects, scope.policy), policy),
            (good, MODULE.Scope("time-core", scope.subjects, "tasks/other.json"), policy),
            (good, MODULE.Scope("time-core", scope.subjects,
                                "policy/time-core-analysis.json"), policy),
            (good, MODULE.Scope("time-core", scope.subjects[:-1], scope.policy), policy),
            (good, MODULE.Scope("time-core", scope.subjects * 2, scope.policy), policy),
            (good, MODULE.Scope("time-core", tuple(reversed(scope.subjects)), scope.policy), policy),
            (good, MODULE.Scope("time-core", tracking_scope.subjects, scope.policy), policy),
            (good, tracking_scope, policy),
            (good, scope, tracking_policy),
        ]
        for lane in ("inventory", "contracts", "coverage", "fuzz", "all", "cohort-union"):
            invalid.append((SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy))
        for profile in (None, True, "", "unknown", "full"):
            invalid.append((SimpleNamespace(lane="mutation", tool_admission_profile=profile), scope, policy))
        for field, value in (
                ("schema_version", 8), ("schema_version", True), ("schema_version", 9.0),
                ("admission_status", "admitted-four-owner-inventory-only"),
                ("package", "time-domain"), ("source_file_count", 135), ("source_file_count", 136.0),
                ("subjects", {}), ("subjects", tracking_policy["subjects"]),
                ("native_test_invocations", 1), ("native_test_invocations", False),
                ("campaigns_admitted", True), ("campaigns_admitted", 0),
                ("mutation_equivalences", [{}]), ("mutation_equivalences", ()),
                ("tool_admission_profile", "full")):
            invalid.append((good, scope, {**policy, field: value}))
        for field in ("schema_version", "admission_status", "package", "source_file_count", "subjects",
                      "native_test_invocations", "campaigns_admitted", "mutation_equivalences",
                      "tool_admission_profile"):
            invalid.append((good, scope, {key: value for key, value in policy.items() if key != field}))
        for args, candidate_scope, candidate_policy in invalid:
            with self.subTest(args=args, scope=candidate_scope, policy=candidate_policy), \
                    self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, candidate_scope, candidate_policy)

    CHART_ERROR = ("{ let _ = (projection, enclose_labeled_scalars, "
                   "FixtureChartProjection::enclosure_distance_seconds); FixtureChartError::Invariant }")

    @classmethod
    def chart_batch_inventory_control(cls):
        """Independent single-owner card facts; no source, tool or native admission."""
        subjects = (("chart_batch", "crates/time-core/src/chart_batch.rs"),)
        scope = MODULE.Scope("time-core", subjects, "tasks/s1.2-chart-batch-inventory-admission.json")
        return scope, {
            "schema_version": 11,
            "admission_status": "admitted-single-owner-chart-batch-inventory-only",
            "package": "time-core", "source_file_count": 139, "subjects": dict(subjects),
            "mutation_equivalences": [], "mutation_error_values": {},
            "native_test_invocations": 0, "campaigns_admitted": False,
            "tool_admission_profile": "stable-core", "mutation_jobs": 10,
            "mutation_message_format": "json", "mutation_process_limit": 512,
            "maximum_mutants": 512, "listing_maximum_per_file": 512,
            "listing_maximum_total": 512, "inventory_wall_seconds": 600,
            "listing_process_limit": 128,
        }

    def test_chart_batch_listing_requires_its_exact_noncampaign_role(self):
        scope, policy = self.chart_batch_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
        self.assertEqual(MODULE.per_subject_mutation_error_args(
            scope, "crates/time-core/src/chart_batch.rs", policy), [])
        for lane in ("contracts", "coverage", "all", "fuzz", "inventory", "cohort-union"):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"),
                                         scope, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane="mutation", tool_admission_profile="full"),
                                     scope, {**policy, "tool_admission_profile": "full"})

    def test_chart_batch_listing_rejects_changed_and_missing_typed_facts(self):
        scope, policy = self.chart_batch_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        changes = (("schema_version", 10), ("schema_version", True),
                   ("source_file_count", 138), ("native_test_invocations", False),
                   ("campaigns_admitted", 0), ("campaigns_admitted", True),
                   ("mutation_equivalences", [{}]), ("subjects", {}),
                   ("mutation_error_values", None), ("mutation_error_values", []),
                   ("mutation_error_values", {"chart_enclosure": [self.CHART_ERROR]}))
        for key, value in changes:
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in ("schema_version", "admission_status", "package", "source_file_count", "subjects",
                    "mutation_equivalences", "mutation_error_values", "native_test_invocations",
                    "campaigns_admitted", "tool_admission_profile"):
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_chart_batch_scope_cannot_borrow_foreign_paths_or_subjects(self):
        scope, policy = self.chart_batch_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        for wrong in (
            MODULE.Scope("time-domain", scope.subjects, scope.policy),
            MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json"),
            MODULE.Scope(scope.package, scope.subjects, "tasks/s1.2-chart-inventory-admission-v2.json"),
            MODULE.Scope(scope.package, (), scope.policy),
            MODULE.Scope(scope.package, (*scope.subjects, ("other", "src/other.rs")), scope.policy),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)

    def test_chart_batch_empty_operator_map_rejects_missing_and_nonempty_values(self):
        card, policy = self.chart_batch_inventory_control()
        scope = MODULE.Scope(card.package, card.subjects, "policy/time-core-analysis.json")
        filename = "crates/time-core/src/chart_batch.rs"
        self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])
        for value in (None, [], False, {"chart_batch": []}, {"chart_batch": ["false"]},
                      {"chart_enclosure": [self.CHART_ERROR]}):
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename,
                                                       {**policy, "mutation_error_values": value})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.per_subject_mutation_error_args(scope, filename,
                                                   {k: v for k, v in policy.items() if k != "mutation_error_values"})
        self.assertEqual(MODULE.per_subject_mutation_error_args(SCOPE, FILES[0][1], {}), [])

    def test_chart_batch_listing_and_campaign_keep_no_extra_operator(self):
        card, policy = self.chart_batch_inventory_control()
        scope = MODULE.Scope(card.package, card.subjects, "policy/time-core-analysis.json")
        tools = Path("/sealed/tools")
        prefix = [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file",
                  "crates/time-core/src/chart_batch.rs", "--test-workspace", "false", "--cap-lints", "false",
                  "--jobs", "10", "--jobserver", "true", "--jobserver-tasks", "10",
                  "--cargo-arg=--message-format=json"]
        calls = []
        def run(label, command, timeout):
            self.assertEqual(command, prefix + ["--list", "--json"])
            calls.append(label)
            return json.dumps([{"name": "one", "file": "crates/time-core/src/chart_batch.rs", "package": "time-core"}])
        found = MODULE.enumerate_mutation_inventories(
            scope, dict(scope.subjects), policy, tools, run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=512)
        self.assertEqual(calls, ["mutation-enumeration:chart_batch"])
        command = [*found.commands["chart_batch"], "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                   "--output", "/work/mutation/chart_batch"]
        MODULE.mutation_campaign_command(scope, tools, "chart_batch", command,
                                        expected_jobs=10, expected_message_format="json", policy=policy)
        with self.assertRaisesRegex(MODULE.Incomplete, "listing only"):
            MODULE.mutation_campaign_command(card, tools, "chart_batch", command,
                                            expected_jobs=10, expected_message_format="json", policy=policy)
        wrong = command.copy()
        wrong[wrong.index("--baseline"):wrong.index("--baseline")] = ["--error", "false"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_campaign_command(scope, tools, "chart_batch", wrong,
                                            expected_jobs=10, expected_message_format="json", policy=policy)

    @staticmethod
    def chart_principal_inventory_control():
        """Literal three-owner S157 listing role; native admission remains separate."""
        subjects = (("principal", "crates/time-core/src/principal.rs"),
                    ("chart_principal", "crates/time-core/src/principal/chart.rs"),
                    ("chart_batch", "crates/time-core/src/chart_batch.rs"))
        scope = MODULE.Scope("time-core", subjects, "tasks/s1.3-chart-principal-inventory-admission.json")
        return scope, {
            "schema_version": 12, "admission_status": "admitted-retained-chart-three-owner-inventory-only",
            "package": "time-core", "source_file_count": 142, "subjects": dict(subjects),
            "mutation_equivalences": [], "mutation_error_values": {},
            "native_test_invocations": 0, "campaigns_admitted": False,
            "tool_admission_profile": "stable-core", "mutation_jobs": 10,
            "mutation_message_format": "json", "maximum_mutants": 512,
        }

    @staticmethod
    def chart_combination_inventory_control():
        """Independent S158 nine-owner noncampaign facts; no native execution."""
        subjects = (
            ("combination", "crates/time-core/src/combination.rs"),
            ("combination_admission", "crates/time-core/src/combination/admission.rs"),
            ("combination_lifecycle", "crates/time-core/src/combination/lifecycle.rs"),
            ("chart", "crates/time-core/src/regression/chart.rs"),
            ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
            ("chart_combination_roundoff", "crates/time-core/src/combination/chart/roundoff.rs"),
            ("chart_combination_receiving", "crates/time-core/src/combination/chart/receiving.rs"),
            ("chart_combination_reduction", "crates/time-core/src/combination/chart/reduction.rs"),
            ("chart_combination_lifecycle", "crates/time-core/src/combination/lifecycle/chart.rs"),
        )
        scope = MODULE.Scope("time-core", subjects, "tasks/s1.3-chart-combination-inventory-admission.json")
        return scope, {
            "schema_version": 13, "admission_status": "admitted-chart-combination-nine-owner-inventory-only",
            "package": "time-core", "source_file_count": 152, "subjects": dict(subjects),
            "mutation_equivalences": [], "mutation_error_values": {},
            "native_test_invocations": 0, "campaigns_admitted": False,
            "tool_admission_profile": "stable-core", "mutation_jobs": 10,
            "mutation_message_format": "json", "mutation_process_limit": 512,
            "listing_process_limit": 128, "maximum_mutants": 512,
            "listing_maximum_per_file": 512, "listing_maximum_total": 4608,
            "inventory_wall_seconds": 600,
        }

    @classmethod
    def chart_combination_campaign_error_control(cls):
        """Isolate role/list shape; this opaque record has no proof admission."""
        inventory, _ = cls.chart_combination_inventory_control()
        return MODULE.Scope(inventory.package, inventory.subjects, 'policy/time-core-analysis.json'), {
            'package': 'time-core', 'admission_status': 'admitted-final-source',
            'mutation_error_values': {},
            'mutation_equivalences': [{'id': 'synthetic-unadmitted-proof'}],
        }

    def test_chart_combination_listing_admits_exact_nine_owner_noncampaign_role(self):
        scope, policy = self.chart_combination_inventory_control()
        self.assertEqual(MODULE.admission_profile(
            SimpleNamespace(lane="mutation", tool_admission_profile="stable-core"), scope, policy), "stable-core")
        current = MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json")
        for filename in dict(scope.subjects).values():
            with self.subTest(filename=filename):
                self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])
                self.assertEqual(MODULE.per_subject_mutation_error_args(
                    current, filename, {**policy, 'admission_status': 'admitted-final-source'}), [])

    def test_chart_combination_listing_rejects_scope_lane_profile_and_typed_fact_changes(self):
        scope, policy = self.chart_combination_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        for wrong in (
            MODULE.Scope("time-domain", scope.subjects, scope.policy),
            MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
            MODULE.Scope(scope.package, scope.subjects[:-1], scope.policy),
            MODULE.Scope(scope.package, (*scope.subjects, ('other', 'src/other.rs')), scope.policy),
            MODULE.Scope(scope.package, scope.subjects, 'policy/time-core-analysis.json'),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/s1.3-chart-principal-inventory-admission.json'),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json'),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)
        for lane in ('contracts', 'coverage', 'all', 'fuzz', 'inventory', 'cohort-union'):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='full'),
                                     scope, {**policy, 'tool_admission_profile': 'full'})
        for key, value in (
            ('schema_version', 12), ('schema_version', True), ('source_file_count', 150),
            ('source_file_count', 152.0), ('native_test_invocations', False),
            ('native_test_invocations', 1), ('campaigns_admitted', 0), ('campaigns_admitted', True),
            ('subjects', {}), ('package', 'time-domain'), ('mutation_equivalences', [{}]),
            ('mutation_error_values', None), ('mutation_error_values', []),
            ('mutation_error_values', {'chart_combination_reduction': ['false']}),
            ('maximum_mutants', 513), ('listing_maximum_per_file', True),
            ('listing_maximum_total', 512), ('listing_maximum_total', 4608.0),
            ('listing_process_limit', 512), ('mutation_process_limit', 128),
            ('mutation_jobs', 12), ('mutation_jobs', 10.0), ('mutation_message_format', 'human'),
            ('inventory_wall_seconds', 601),
            ('admission_status', 'admitted-retained-chart-three-owner-inventory-only'),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_chart_combination_error_maps_require_exact_role_and_explicit_empty_mapping(self):
        card, card_policy = self.chart_combination_inventory_control()
        current = MODULE.Scope(card.package, card.subjects, 'policy/time-core-analysis.json')
        current_policy = {**card_policy, 'admission_status': 'admitted-final-source'}
        for scope, policy in ((card, card_policy), (current, current_policy)):
            for filename in dict(card.subjects).values():
                for value in (None, [], False, {'chart_combination_reduction': []},
                              {'chart_combination_reduction': ['false']}, {'chart_enclosure': [self.CHART_ERROR]}):
                    with self.subTest(scope=scope, filename=filename, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.per_subject_mutation_error_args(scope, filename, {**policy, 'mutation_error_values': value})
                with self.subTest(scope=scope, filename=filename, missing=True), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(
                        scope, filename, {k: v for k, v in policy.items() if k != 'mutation_error_values'})
            for filename in ('src/other.rs', 'crates/time-core/src/principal.rs', None):
                with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, policy)
            for field, value in (('admission_status', 'foreign-role'),):
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, card.subjects[0][1], {**policy, field: value})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.per_subject_mutation_error_args(
                card, card.subjects[0][1], {**card_policy, 'mutation_equivalences': [{}]})
        for wrong in (
            MODULE.Scope('time-domain', card.subjects, card.policy),
            MODULE.Scope(card.package, tuple(reversed(card.subjects)), card.policy),
            MODULE.Scope(card.package, card.subjects[:-1], card.policy),
            MODULE.Scope(card.package, card.subjects, 'tasks/renamed.json'),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(wrong, card.subjects[0][1], card_policy)
        self.assertEqual(MODULE.per_subject_mutation_error_args(SCOPE, FILES[0][1], {}), [])

    def test_chart_combination_final_source_proof_metadata_adds_no_rust_error_operator(self):
        scope, policy = self.chart_combination_campaign_error_control()
        # This helper consumes only the already-selected role and list shape.
        # Core/Common admission separately rejects this synthetic non-proof.
        for filename in dict(scope.subjects).values():
            with self.subTest(filename=filename):
                self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])
                self.assertEqual(MODULE.per_subject_mutation_error_args(
                    scope, filename, {**policy, 'mutation_equivalences': []}), [])
        inventory, inventory_policy = self.chart_combination_inventory_control()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.per_subject_mutation_error_args(inventory, inventory.subjects[0][1],
                {**inventory_policy, 'mutation_equivalences': policy['mutation_equivalences']})

    def test_chart_combination_nonempty_proof_list_cannot_relax_error_map_or_role(self):
        scope, policy = self.chart_combination_campaign_error_control()
        filename = scope.subjects[0][1]
        for value in (None, [], False, True, 1, '{}', {'chart_combination_receiving': []},
                      {'chart_combination_receiving': ['false']}, {'chart_enclosure': [self.CHART_ERROR]}):
            with self.subTest(error_map=value), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename, {**policy, 'mutation_error_values': value})
        for value in (None, {}, (), False, True, 0, '[]'):
            with self.subTest(proof_collection=value), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename, {**policy, 'mutation_equivalences': value})
        for field in ('mutation_error_values', 'mutation_equivalences', 'admission_status', 'package'):
            with self.subTest(missing=field), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename,
                    {key: value for key, value in policy.items() if key != field})
        for status in ('foreign-role', 'admitted-chart-combination-nine-owner-inventory-only',
                       'admitted-retained-chart-three-owner-inventory-only'):
            with self.subTest(status=status), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename, {**policy, 'admission_status': status})
        for wrong in (
            MODULE.Scope('time-domain', scope.subjects, scope.policy),
            MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
            MODULE.Scope(scope.package, scope.subjects[:-1], scope.policy),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/s1.3-chart-combination-inventory-admission.json'),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json'),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(wrong, filename, policy)
        for wrong in ('src/other.rs', 'crates/time-core/src/principal.rs', None):
            with self.subTest(filename=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, wrong, policy)

    def test_chart_combination_listing_commands_add_no_operator_and_cannot_campaign(self):
        card, policy = self.chart_combination_inventory_control()
        calls = []
        def run(label, command, timeout):
            self.assertEqual(command[-2:], ['--list', '--json'])
            self.assertNotIn('--error', command)
            self.assertNotIn('--baseline', command)
            calls.append(label)
            return json.dumps([{'name': label, 'file': command[command.index('--file') + 1],
                                'package': 'time-core'}])
        found = MODULE.enumerate_mutation_inventories(
            card, dict(card.subjects), policy, Path('/sealed/tools'), run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=4608)
        self.assertEqual(len(calls), 9)
        self.assertEqual(set(found.inventories), set(dict(card.subjects)))
        for subject, listing in found.commands.items():
            command = [*listing, '--baseline', 'run', '--build-timeout', '90', '--timeout', '60',
                       '--output', '/work/mutation/' + subject]
            with self.subTest(subject=subject), self.assertRaisesRegex(MODULE.Incomplete, 'listing only'):
                MODULE.mutation_campaign_command(card, Path('/sealed/tools'), subject, command,
                    expected_jobs=10, expected_message_format='json', policy=policy)

    def test_chart_principal_listing_admits_exact_card_and_explicit_empty_map(self):
        scope, policy = self.chart_principal_inventory_control()
        self.assertEqual(MODULE.admission_profile(
            SimpleNamespace(lane="mutation", tool_admission_profile="stable-core"), scope, policy), "stable-core")
        self.assertEqual(MODULE.CHART_PRINCIPAL_SUBJECTS, scope.subjects)
        for filename in dict(scope.subjects).values():
            with self.subTest(filename=filename):
                self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])
                current = MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json")
                self.assertEqual(MODULE.per_subject_mutation_error_args(current, filename, policy), [])

    def test_chart_principal_listing_rejects_scope_lane_and_typed_fact_changes(self):
        scope, policy = self.chart_principal_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        wrong_scopes = (
            MODULE.Scope("time-domain", scope.subjects, scope.policy),
            MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
            MODULE.Scope(scope.package, scope.subjects[:1], scope.policy),
            MODULE.Scope(scope.package, (*scope.subjects, ("other", "src/other.rs")), scope.policy),
            MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json"),
            MODULE.Scope(scope.package, scope.subjects, "tasks/s1.2-chart-batch-inventory-admission.json"),
            MODULE.Scope(scope.package, scope.subjects, "tasks/renamed-card.json"),
        )
        for wrong in wrong_scopes:
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)
        for lane in ("contracts", "coverage", "all", "fuzz", "inventory", "cohort-union"):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane="mutation", tool_admission_profile="full"),
                                     scope, {**policy, "tool_admission_profile": "full"})
        changes = (("schema_version", 11), ("schema_version", True),
                   ("source_file_count", 141), ("native_test_invocations", False),
                   ("campaigns_admitted", 0), ("campaigns_admitted", True),
                   ("mutation_equivalences", [{}]), ("subjects", {}), ("package", "time-domain"),
                   ("mutation_error_values", None), ("mutation_error_values", []),
                   ("mutation_error_values", {"chart_principal": ["false"]}),
                   ("admission_status", "admitted-single-owner-chart-batch-inventory-only"))
        for key, value in changes:
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in ("schema_version", "admission_status", "package", "source_file_count", "subjects",
                    "mutation_equivalences", "mutation_error_values", "native_test_invocations",
                    "campaigns_admitted", "tool_admission_profile"):
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_chart_principal_error_map_requires_exact_role_owner_and_explicit_empty_mapping(self):
        card, policy = self.chart_principal_inventory_control()
        current = MODULE.Scope(card.package, card.subjects, "policy/time-core-analysis.json")
        for scope in (card, current):
            for filename in dict(scope.subjects).values():
                for value in (None, [], False, {"chart_principal": []}, {"chart_principal": ["false"]},
                              {"chart_enclosure": [self.CHART_ERROR]}):
                    with self.subTest(scope=scope, filename=filename, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.per_subject_mutation_error_args(scope, filename,
                                                               {**policy, "mutation_error_values": value})
                with self.subTest(scope=scope, filename=filename, missing=True), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename,
                        {k: v for k, v in policy.items() if k != "mutation_error_values"})
            for filename in ("src/other.rs", "crates/time-core/src/combination.rs", None):
                with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, policy)
        for scope in (
            MODULE.Scope("time-domain", card.subjects, card.policy),
            MODULE.Scope(card.package, tuple(reversed(card.subjects)), card.policy),
            MODULE.Scope(card.package, card.subjects[:1], card.policy),
            MODULE.Scope(card.package, card.subjects, "tasks/renamed-card.json"),
        ):
            with self.subTest(scope=scope), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, "crates/time-core/src/principal/chart.rs", policy)
            with self.subTest(scope=scope, missing=True), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, "crates/time-core/src/principal/chart.rs",
                    {k: v for k, v in policy.items() if k != "mutation_error_values"})
        self.assertEqual(MODULE.per_subject_mutation_error_args(SCOPE, FILES[0][1], {}), [])

    def test_chart_principal_listing_commands_are_exact_and_card_cannot_start_campaign(self):
        card, policy = self.chart_principal_inventory_control()
        current = MODULE.Scope(card.package, card.subjects, "policy/time-core-analysis.json")
        tools = Path("/sealed/tools")
        calls = []
        def run(label, command, timeout):
            subject = label.removeprefix("mutation-enumeration:")
            filename = dict(card.subjects)[subject]
            expected = [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file", filename,
                        "--test-workspace", "false", "--cap-lints", "false", "--jobs", "10",
                        "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
                        "--list", "--json"]
            self.assertEqual(command, expected)
            calls.append(label)
            return json.dumps([{"name": "one", "file": filename, "package": "time-core"}])
        found = MODULE.enumerate_mutation_inventories(
            card, dict(card.subjects), policy, tools, run, {}, deadline=MODULE.time.monotonic() + 60,
            maximum_per_file=512, maximum_total=1536)
        self.assertEqual(calls, ["mutation-enumeration:" + name for name, _ in card.subjects])
        for subject, _ in card.subjects:
            command = [*found.commands[subject], "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                       "--output", "/work/mutation/" + subject]
            MODULE.mutation_campaign_command(current, tools, subject, command,
                expected_jobs=10, expected_message_format="json", policy=policy)
            with self.subTest(subject=subject), self.assertRaisesRegex(MODULE.Incomplete, "listing only"):
                MODULE.mutation_campaign_command(card, tools, subject, command,
                    expected_jobs=10, expected_message_format="json", policy=policy)
            wrong = command.copy()
            wrong[wrong.index("--baseline"):wrong.index("--baseline")] = ["--error", "false"]
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(current, tools, subject, wrong,
                    expected_jobs=10, expected_message_format="json", policy=policy)

    @classmethod
    def chart_error_inventory_control(cls):
        """Independent exact new listing-only role; no tool admission is run."""
        scope, policy = cls.chart_inventory_control()
        scope = MODULE.Scope(scope.package, scope.subjects,
                             "tasks/s1.2-chart-inventory-admission-v2.json")
        policy = {**policy, "schema_version": 10,
                  "admission_status": "admitted-four-owner-chart-error-inventory-only",
                  "mutation_error_values": {"chart_enclosure": [cls.CHART_ERROR]}}
        return scope, policy

    def test_chart_error_inventory_role_admits_exact_card_and_rejects_full_fallback(self):
        scope, policy = self.chart_error_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
        for profile in ("full",):
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane="mutation", tool_admission_profile=profile),
                                         scope, {**policy, "tool_admission_profile": profile})

    def test_chart_error_inventory_rejects_changed_role_and_typed_error_map(self):
        scope, policy = self.chart_error_inventory_control()
        args = SimpleNamespace(lane="mutation", tool_admission_profile="stable-core")
        bad = [
            (MODULE.Scope("time-domain", scope.subjects, scope.policy), policy),
            (MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy), policy),
            (MODULE.Scope(scope.package, scope.subjects[:1], scope.policy), policy),
            (MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json"), policy),
            (MODULE.Scope(scope.package, scope.subjects, "tasks/s1.2-chart-inventory-admission.json"), policy),
        ]
        for field, value in (("schema_version", 9), ("schema_version", True),
                             ("source_file_count", 135), ("native_test_invocations", False),
                             ("campaigns_admitted", 0), ("campaigns_admitted", True),
                             ("admission_status", "admitted-four-owner-chart-inventory-only"),
                             ("subjects", dict(scope.subjects[:1])),
                             ("mutation_error_values", {}), ("mutation_error_values", None),
                             ("mutation_error_values", {"chart_enclosure": self.CHART_ERROR}),
                             ("mutation_error_values", {"chart_enclosure": [self.CHART_ERROR, self.CHART_ERROR]}),
                             ("mutation_error_values", {"chart": [self.CHART_ERROR]})):
            bad.append((scope, {**policy, field: value}))
        for field in ("schema_version", "source_file_count", "native_test_invocations", "campaigns_admitted",
                      "mutation_error_values", "admission_status", "subjects", "tool_admission_profile"):
            bad.append((scope, {key: value for key, value in policy.items() if key != field}))
        for candidate_scope, candidate_policy in bad:
            with self.subTest(scope=candidate_scope, policy=candidate_policy), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, candidate_scope, candidate_policy)
        for lane in ("inventory", "contracts", "coverage", "fuzz", "all", "cohort-union"):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)

    def test_chart_error_listing_and_campaign_share_one_exact_argument(self):
        card_scope, policy = self.chart_error_inventory_control()
        scope = MODULE.Scope(card_scope.package, card_scope.subjects, "policy/time-core-analysis.json")
        tools = Path("/sealed/tools"); calls = []
        def run(label, command, _timeout):
            name = label.removeprefix("mutation-enumeration:")
            calls.append((name, command))
            expected = ["--error", self.CHART_ERROR] if name == "chart_enclosure" else []
            prefix = [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file", dict(scope.subjects)[name],
                      "--test-workspace", "false", "--cap-lints", "false", "--jobs", "10",
                      "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json"]
            self.assertEqual(command, prefix + expected + ["--list", "--json"])
            return json.dumps([{"name": name, "file": dict(scope.subjects)[name], "package": "time-core"}])
        found = MODULE.enumerate_mutation_inventories(scope, dict(scope.subjects), policy, tools, run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=2048)
        self.assertEqual(len(calls), 4)
        for name, prefix in found.commands.items():
            campaign = [*prefix, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                        "--output", "/work/mutation/" + name]
            MODULE.mutation_campaign_command(scope, tools, name, campaign, expected_jobs=10,
                                             expected_message_format="json", policy=policy)
            wrong = campaign.copy()
            if name == "chart_enclosure": del wrong[wrong.index("--error"):wrong.index("--error") + 2]
            else: wrong[wrong.index("--baseline"):wrong.index("--baseline")] = ["--error", self.CHART_ERROR]
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(scope, tools, name, wrong, expected_jobs=10,
                                                 expected_message_format="json", policy=policy)

    def test_chart_error_session_rejects_missing_or_substituted_args_before_native(self):
        scope, policy = self.chart_error_inventory_control()
        scope = MODULE.Scope(scope.package, scope.subjects, "policy/time-core-analysis.json")
        with self.stable_session() as session:
            session.scope = scope; session.policy = policy
            prefix = [str(session.tools / "cargo-mutants"), "mutants", "--package", "time-core",
                      "--file", dict(scope.subjects)["chart_enclosure"], "--test-workspace", "false", "--cap-lints", "false",
                      "--jobs", "10", "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json"]
            tail = ["--baseline", "run", "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/chart_enclosure"]
            success = {"exit_code": 0, "failure": None, "output": "synthetic complete result"}
            for arguments in ([], ["--error", "FixtureChartError::Invariant"],
                              ["--error", self.CHART_ERROR, "--error", self.CHART_ERROR]):
                with self.subTest(arguments=arguments), patch.object(MODULE.RUNTIME, "run", return_value=success) as native:
                    with self.assertRaises(MODULE.Incomplete):
                        session.run("mutation-campaign:chart_enclosure", prefix + arguments + tail, 1,
                                    mutation_subject="chart_enclosure")
                    native.assert_not_called()
            with patch.object(MODULE.RUNTIME, "run", return_value=success) as native:
                self.assertEqual(session.run("mutation-campaign:chart_enclosure",
                    prefix + ["--error", self.CHART_ERROR] + tail, 1, mutation_subject="chart_enclosure"), success)
                native.assert_called_once()

    def test_chart_error_map_is_strict_and_absence_preserves_legacy_scopes(self):
        scope, policy = self.chart_error_inventory_control()
        self.assertEqual(MODULE.CHART_ENCLOSURE_ERROR_VALUE, self.CHART_ERROR)
        for name, filename in scope.subjects:
            expected = ["--error", self.CHART_ERROR] if name == "chart_enclosure" else []
            self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), expected)
        self.assertEqual(MODULE.per_subject_mutation_error_args(SCOPE, dict(FILES)["clock"], {}), [])
        wrong_scopes = [SCOPE, MODULE.Scope("other", scope.subjects, scope.policy),
                        MODULE.Scope("time-core", scope.subjects, []),
                        MODULE.Scope("time-core", scope.subjects, "tasks/other.json"),
                        MODULE.Scope("time-core", (("other", dict(scope.subjects)["chart_enclosure"]),), scope.policy)]
        for candidate in wrong_scopes:
            with self.subTest(scope=candidate), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(candidate, dict(scope.subjects)["chart_enclosure"], policy)
        class DictSubclass(dict): pass
        class ListSubclass(list): pass
        bad_maps = [None, [], "bad", {}, True, DictSubclass(policy["mutation_error_values"]),
                    {"chart_enclosure": self.CHART_ERROR}, {"chart_enclosure": (self.CHART_ERROR,)},
                    {"chart_enclosure": ListSubclass([self.CHART_ERROR])}, {"chart_enclosure": []},
                    {"chart_enclosure": [True]}, {"chart_enclosure": [self.CHART_ERROR + " "]},
                    {"chart_enclosure": [self.CHART_ERROR], "chart": [self.CHART_ERROR]}]
        for values in bad_maps:
            with self.subTest(values=values), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, dict(scope.subjects)["chart_enclosure"],
                                                       {**policy, "mutation_error_values": values})
        for package in (None, True, "time-domain"):
            with self.subTest(package=package), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, dict(scope.subjects)["chart_enclosure"],
                                                       {**policy, "package": package})
        for filename in (None, True, "crates/time-core/src/other.rs", "/work/source/" + dict(scope.subjects)["chart_enclosure"]):
            with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename, policy)

    def test_chart_error_result_rederives_policy_and_inventory_cannot_run_campaign(self):
        card_scope, policy = self.chart_error_inventory_control()
        scope = MODULE.Scope(card_scope.package, card_scope.subjects, "policy/time-core-analysis.json")
        tools = Path("/sealed/tools")
        command = [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file",
                   dict(scope.subjects)["chart_enclosure"], "--test-workspace", "false", "--cap-lints", "false",
                   "--jobs", "10", "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
                   "--error", self.CHART_ERROR, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                   "--output", "/work/mutation/chart_enclosure"]
        result = {"exit_code": 0, "failure": None, "output": "synthetic native observation"}
        self.assertIs(MODULE.mutation_campaign_result(scope, tools, "chart_enclosure", command, result,
                      expected_jobs=10, expected_message_format="json", policy=policy), result)
        for candidate in ({key: value for key, value in policy.items() if key != "mutation_error_values"},
                          {**policy, "mutation_error_values": {"chart_enclosure": [self.CHART_ERROR + " "]}}):
            with self.subTest(policy=candidate), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(scope, tools, "chart_enclosure", command, result,
                    expected_jobs=10, expected_message_format="json", policy=candidate)
        with self.assertRaisesRegex(MODULE.Incomplete, "listing only"):
            MODULE.mutation_campaign_command(card_scope, tools, "chart_enclosure", command,
                expected_jobs=10, expected_message_format="json", policy=policy)
        wrong = command.copy(); wrong[wrong.index("--file") + 1] = dict(scope.subjects)["enclosure"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_campaign_command(scope, tools, "chart_enclosure", wrong,
                expected_jobs=10, expected_message_format="json", policy=policy)

    @contextmanager
    def stable_session(self):
        """Real private input trees; only external product/native operations are stubbed."""
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.root, session.output = parent / "checkout", parent / "output"
            session.root.mkdir(); session.output.mkdir()
            session.work = session.output / "work"; session.work.mkdir(mode=0o700)
            session.source = session.work / "source"; session.source.mkdir()
            (session.root / "input").write_text("frozen")
            (session.source / "input").write_text("frozen")
            session.identity = {"input": MODULE.PRODUCT.sha256(session.root / "input")}
            session.product_identity = dict(session.identity)
            session.source_inventory = MODULE.PRODUCT.inventory(session.source)
            session.config = session.work / "config.toml"; session.config.write_text("fixed")
            session.config_sha256 = MODULE.PRODUCT.sha256(session.config)
            session.base, session.stable = parent / "omitted", parent / "stable"
            session.base.mkdir(); session.stable.mkdir()
            for name in MODULE.ANALYSIS.STABLE_CORE_SCOPES:
                (session.stable / name).mkdir()
            session.vendor, session.tools = parent / "vendor", parent / "tools"
            session.vendor.mkdir(); session.tools.mkdir()
            (session.tools / "selected-extension").write_text("sealed")
            session.protected_tools = {session.tools: MODULE.PRODUCT.inventory(session.tools)}
            session.protected_islands = {}
            session.admission_profile = "stable-core"
            session.payload_readonly = tuple(session.stable / name for name in MODULE.ANALYSIS.STABLE_CORE_SCOPES)
            session.admitted = {"base": session.base, "stable": session.stable,
                               "policy": {"common_nightly": "common/nightly"},
                               "policy_sha256": "a" * 64, "inventories": [{"root": "stable"}],
                               "admission_profile": "stable-core", "readonly_roots": session.payload_readonly,
                               "installation_pointer": str(parent / "pointer"),
                               "installation_pointer_sha256": "b" * 64}
            session.dependency_policy, session.vendor_identity = {}, {}
            session.policy = {"tool_admission_profile": "stable-core"}
            session.report = {"lanes": []}
            with patch.object(MODULE, "require_current_candidate"):
                yield session

    def test_stable_core_run_omits_base_and_rejects_compiler_and_island_escalation(self):
        with self.stable_session() as session:
            success = {"exit_code": 0, "failure": None, "output": "healthy"}
            with patch.object(MODULE.RUNTIME, "run", return_value=success) as native:
                self.assertEqual(session.run("control", ["fixture"], 1), "healthy")
                self.assertEqual(native.call_args.kwargs["readonly"],
                                 [*session.payload_readonly, session.vendor, session.tools])
                self.assertNotIn(session.base, native.call_args.kwargs["readonly"])
                self.assertNotIn(session.stable, native.call_args.kwargs["readonly"])
                native.reset_mock()
                session.protected_tools[session.base] = MODULE.PRODUCT.inventory(session.base)
                for options in ({"compiler": session.base / "common/nightly"}, {"extra_tools": session.base}):
                    with self.subTest(options=options), self.assertRaisesRegex(MODULE.Incomplete, "nightly compiler or extra"):
                        session.run("control", ["fixture"], 1, **options)
                native.assert_not_called()

    def test_stable_core_keeps_pre_and_post_command_sealed_input_guards(self):
        with self.stable_session() as session:
            with patch.object(MODULE.RUNTIME, "run") as native:
                (session.tools / "undeclared").write_text("unadmitted dependency")
                with self.assertRaisesRegex(MODULE.Incomplete, "sealed tool"):
                    session.run("control", ["fixture"], 1)
                native.assert_not_called()
                (session.tools / "undeclared").unlink()
            def change_after_launch(*args, **kwargs):
                session.config.write_text("changed")
                return {"exit_code": 0, "failure": None, "output": "healthy"}
            with patch.object(MODULE.RUNTIME, "run", side_effect=change_after_launch) as native:
                with self.assertRaisesRegex(MODULE.Incomplete, "Cargo configuration"):
                    session.run("control", ["fixture"], 1)
                native.assert_called_once()

    def test_stable_core_finish_rehashes_same_profile_and_rejects_pointer_drift(self):
        with self.stable_session() as session:
            with patch.object(MODULE.PRODUCT, "admit_vendor", return_value={}), \
                 patch.object(MODULE.ANALYSIS, "admit", return_value=dict(session.admitted)) as admit:
                session.finish()
                admit.assert_called_once_with(session.root, profile="stable-core")
                self.assertTrue(session.report["analysis_admission_after"]["unchanged"])
            changed = {**session.admitted, "installation_pointer_sha256": "c" * 64}
            with patch.object(MODULE.PRODUCT, "admit_vendor", return_value={}), \
                 patch.object(MODULE.ANALYSIS, "admit", return_value=changed):
                with self.assertRaisesRegex(MODULE.Incomplete, "installation/payload"):
                    session.finish()

    def test_mutation_json_policy_emits_flag_in_listing_and_campaign(self):
        """Core's JSON mode reaches both commands and exact command guards."""
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                      "mutation_jobs": 2, "mutation_message_format": "json"}
            calls = []
            def run(label, command, timeout, **options):
                calls.append((label, command))
                self.assertEqual(command.count("--cargo-arg=--message-format=json"), 1)
                self.assertLess(command.index("--cargo-arg=--message-format=json"),
                                command.index("--list" if label.startswith("mutation-enumeration:") else "--baseline"))
                mutant = {"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return json.dumps([mutant])
                self.assertEqual(options, {"mutation_subject": "era"})
                directory = work / "mutation/era/mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                def json_phases(summary):
                    phases = phase_records(summary)
                    for phase in phases:
                        phase["argv"].insert(phase["argv"].index("--frozen"), "--message-format=json")
                    return phases
                (directory / "outcomes.json").write_text(json.dumps(complete_report({
                    "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log",
                                  "phase_results": json_phases("Success")},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant",
                                  "phase_results": json_phases("CaughtMutant")}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy, work, tools, run, {}, "mutation")
            self.assertEqual([label for label, _ in calls], ["mutation-enumeration:era", "mutation-campaign:era"])
            command = calls[1][1]
            MODULE.mutation_campaign_command(SCOPE, tools, "era", command,
                                             expected_jobs=2, expected_message_format="json")
            result = {"exit_code": 2, "failure": None, "output": "survivor"}
            self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                         expected_jobs=2, expected_message_format="json"), result)
            without_flag = [item for item in command if item != "--cargo-arg=--message-format=json"]
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(SCOPE, tools, "era", without_flag,
                                                 expected_jobs=2, expected_message_format="json")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                                                expected_jobs=2, expected_message_format="human")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(SCOPE, tools, "era", command,
                                                 expected_jobs=2, expected_message_format=True)

    def test_invalid_mutation_message_format_stops_before_command(self):
        self.assertEqual(MODULE.mutation_message_format({}), "human")
        self.assertEqual(MODULE.mutation_message_format({"mutation_message_format": "json"}), "json")
        for value in ("", "JSON", "xml", None, True, 2):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_message_format": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_phases_accept_exact_json_or_legacy_human_arguments(self):
        for summary in ("Success", "CaughtMutant", "Unviable"):
            human = {"summary": summary, "phase_results": phase_records(summary)}
            MODULE.mutation_phases(SCOPE, human)
            encoded = json.loads(json.dumps(human))
            for phase in encoded["phase_results"]:
                phase["argv"].insert(phase["argv"].index("--frozen"), "--message-format=json")
            MODULE.mutation_phases(SCOPE, encoded)
            corrupted = json.loads(json.dumps(encoded))
            corrupted["phase_results"][0]["argv"].insert(-1, "--message-format=human")
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_phases(SCOPE, corrupted)
            if len(encoded["phase_results"]) == 2:
                mixed = json.loads(json.dumps(encoded))
                mixed["phase_results"][0]["argv"].remove("--message-format=json")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_phases(SCOPE, mixed)

    def test_mutation_policy_uses_two_workers_for_enumeration_and_campaign(self):
        """The selected policy controls both real command arguments and guard."""
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512, "mutation_jobs": 2}
            calls = []
            def run(label, command, timeout, **options):
                calls.append((label, command))
                self.assertEqual(command[command.index("--jobs") + 1], "2")
                self.assertEqual(command[command.index("--jobserver-tasks") + 1], "2")
                mutant = {"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return json.dumps([mutant])
                self.assertEqual(options, {"mutation_subject": "era"})
                directory = work / "mutation/era/mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({
                    "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            report = {}
            MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy, work, tools, run, report, "mutation")
            self.assertEqual([label for label, _ in calls], ["mutation-enumeration:era", "mutation-campaign:era"])
            command = calls[1][1]
            MODULE.mutation_campaign_command(SCOPE, tools, "era", command, expected_jobs=2)
            result = {"exit_code": 2, "failure": None, "output": "survivor"}
            self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result,
                                                          expected_jobs=2), result)
            for index in (command.index("--jobs") + 1, command.index("--jobserver-tasks") + 1):
                changed = list(command); changed[index] = "12"
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_command(SCOPE, tools, "era", changed, expected_jobs=2)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_result(SCOPE, tools, "era", changed, result, expected_jobs=2)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result, expected_jobs=12)
            for invalid in (True, 0, 13, "2"):
                with self.subTest(invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_campaign_command(SCOPE, tools, "era", command, expected_jobs=invalid)

    def test_invalid_mutation_worker_counts_stop_before_command(self):
        self.assertEqual(MODULE.mutation_jobs({}), 12)
        self.assertEqual(MODULE.mutation_jobs({"mutation_jobs": 1}), 1)
        self.assertEqual(MODULE.mutation_jobs({"mutation_jobs": 12}), 12)
        for value in (True, False, 0, -1, 13, "2", 2.0, None):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512, "mutation_jobs": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_process_profiles_are_fixed_and_invalid_values_stop_before_command(self):
        self.assertEqual(MODULE.mutation_process_limit({}), 128)
        self.assertEqual(MODULE.mutation_process_limit({"mutation_process_limit": 128}), 128)
        self.assertEqual(MODULE.mutation_process_limit({"mutation_process_limit": 512}), 512)
        for value in (True, False, None, "512", 512.0, 0, -1, 127, 129, 256, 513, 1024):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                run = unittest.mock.Mock()
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_process_limit": value}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                              work, work / "tools", run, {}, "mutation")
                run.assert_not_called()

    def test_mutation_campaign_selects_worker_target_isolation(self):
        """The guarded campaign must reach the runtime's worker-owned target mode."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.check = lambda: None
            session.root = root
            session.work = root / "work"
            session.output = root
            session.base = root / "base"
            session.stable = root / "stable"
            session.admission_profile = "full"
            session.payload_readonly = (session.base, session.stable)
            session.vendor = root / "vendor"
            session.tools = root / "tools"
            session.source = root / "source"
            session.config = root / "config"
            session.protected_islands = {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.report = {"lanes": []}
            session.scope = SCOPE
            session.policy = {"mutation_jobs": 2, "mutation_message_format": "json"}
            result = {"exit_code": 0, "failure": None, "output": "owned campaign", "command": ["fixture"]}
            with (patch.object(MODULE, "mutation_campaign_command") as admission,
                  patch.object(MODULE, "mutation_campaign_result", return_value="checked outcome") as checked_result,
                  patch.object(MODULE.RUNTIME, "run", return_value=result) as run):
                self.assertEqual(session.run("mutation-campaign:clock", ["fixture"], 1,
                                             mutation_subject="clock"), "checked outcome")
                admission.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"],
                                                  expected_jobs=2, expected_message_format="json", policy=session.policy)
                checked_result.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"], result,
                                                       expected_jobs=2, expected_message_format="json", policy=session.policy)
                self.assertEqual(run.call_args.kwargs["cargo_target_mode"], "worker")
                self.assertEqual(run.call_args.kwargs["process_limit"], 128)
                self.assertEqual(run.call_args.kwargs["build_jobs"], 1)
                self.assertEqual(run.call_args.kwargs["environment"]["RUST_TEST_THREADS"], "1")
            session.policy["mutation_process_limit"] = 512
            with (patch.object(MODULE, "mutation_campaign_command") as admission,
                  patch.object(MODULE, "mutation_campaign_result", return_value="checked outcome"),
                  patch.object(MODULE.RUNTIME, "run", return_value=result) as run):
                self.assertEqual(session.run("mutation-campaign:clock", ["fixture"], 1,
                                             mutation_subject="clock"), "checked outcome")
                admission.assert_called_once_with(SCOPE, session.tools, "clock", ["fixture"],
                                                  expected_jobs=2, expected_message_format="json", policy=session.policy)
                self.assertEqual(run.call_args.kwargs["process_limit"], 512)
                self.assertEqual(run.call_args.kwargs["cargo_target_mode"], "worker")
                self.assertEqual(run.call_args.kwargs["build_jobs"], 1)
            with patch.object(MODULE.RUNTIME, "run", return_value=result) as run:
                self.assertEqual(session.run("ordinary-control", ["fixture"], 1), "owned campaign")
                self.assertNotIn("cargo_target_mode", run.call_args.kwargs)
                self.assertNotIn("process_limit", run.call_args.kwargs)
                self.assertEqual(session.run("sanitizer-control", ["fixture"], 1, mode="sanitizer"),
                                 "owned campaign")
                self.assertNotIn("process_limit", run.call_args.kwargs)
            with (patch.object(MODULE, "mutation_campaign_command",
                               side_effect=MODULE.Incomplete("substituted campaign")),
                  patch.object(MODULE.RUNTIME, "run") as run):
                with self.assertRaisesRegex(MODULE.Incomplete, "substituted campaign"):
                    session.run("mutation-campaign:clock", ["fixture"], 1, mutation_subject="clock")
                run.assert_not_called()
            with patch.object(MODULE.RUNTIME, "run") as run:
                for options in ({"mode": "sanitizer"}, {"negative": "borrowed failure"},
                                {"sanitizer_profile": "time-core-history-s12d"}):
                    with self.subTest(options=options), self.assertRaises(MODULE.Incomplete):
                        session.run("mutation-campaign:clock", ["fixture"], 1,
                                    mutation_subject="clock", **options)
                run.assert_not_called()

    def test_legal_consumer_preserves_complete_counter_transport(self):
        """A successful child must not lose the final strict counter delimiter."""
        raw = "legal-core-contract\nCORE_RUNS_PROFILE points=3 inputs=1 selections=2\n"
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.check = lambda: None
            session.root = root
            session.work = root / "work"
            session.output = root
            session.base = root / "base"
            session.stable = root / "stable"
            session.admission_profile = "full"
            session.payload_readonly = (session.base, session.stable)
            session.vendor = root / "vendor"
            session.tools = root / "tools"
            session.source = root / "source"
            session.config = root / "config"
            session.protected_islands = {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.policy = {}
            session.report = {"lanes": []}
            success = {"exit_code": 0, "failure": None, "output": raw, "command": ["fixture"]}
            with patch.object(MODULE.RUNTIME, "run", return_value=success):
                self.assertEqual(session.run("external-legal-execution", ["fixture"], 1), raw)
                self.assertEqual(session.run("ordinary-control", ["fixture"], 1), raw.strip())
            failure = {"exit_code": 1, "failure": None, "output": raw, "command": ["fixture"]}
            with patch.object(MODULE.RUNTIME, "run", return_value=failure):
                with self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                    session.run("external-legal-execution", ["fixture"], 1)

    def test_adaptive_legal_consumer_preserves_retained_output_and_failure_guard(self):
        """Replay actual S148 child text through Session.run without a native child."""
        # Exact 337-byte output from s148-contracts-v1/verification.json, receipt
        # SHA256 6d5f52ea37b165e7cce63f5f310459d3605551968dd20ef27eb6968e63205674.
        raw = (
            "legal-core-contract\n"
            "CORE_ADAPTIVE_CONTRACT_PROFILE retained_points=0 retained_inputs=0 "
            "retained_selections=2 adaptive_observations=3 adaptive_appends=3 "
            "adaptive_histories=1 raw_parameter_validations=0 raw_ordinal_validations=0 "
            "raw_event_validations=0 external_normalizations=1 contexts=1 monotonic=3 "
            "coordinates=1 exchanges=1 getters=29\n"
        )
        expected = {
            "retained_points": 0, "retained_inputs": 0, "retained_selections": 2,
            "adaptive_observations": 3, "adaptive_appends": 3, "adaptive_histories": 1,
            "raw_parameter_validations": 0, "raw_ordinal_validations": 0,
            "raw_event_validations": 0, "external_normalizations": 1,
        }
        contracts = MODULE.load_module("adaptive_transport_contracts", ROOT / "scripts/core-adaptive-contracts.py")
        self.assertEqual(len(raw.encode("utf-8")), 337)
        self.assertEqual(contracts.contract_counts(raw), expected)
        with self.assertRaises(contracts.Incomplete):
            contracts.contract_counts(raw.strip())
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.check = unittest.mock.Mock()
            session.root = root
            session.work = root / "work"
            session.output = root
            session.base = root / "base"
            session.stable = root / "stable"
            session.admission_profile = "stable-core"
            session.payload_readonly = (session.base, session.stable)
            session.vendor = root / "vendor"
            session.tools = root / "tools"
            session.source = root / "source"
            session.config = root / "config"
            session.protected_islands = {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.policy = {}
            session.report = {"lanes": []}
            success = {"exit_code": 0, "failure": None, "output": raw, "command": ["fixture"]}
            with patch.object(MODULE.RUNTIME, "run", return_value=success):
                # Transport preserves bytes; each contract owner separately
                # interprets its own record. This payload is the existing
                # adaptive owner control, not a chart semantic observation.
                for index, label in enumerate(("external-adaptive-legal-execution",
                                               "external-chart-legal-execution",
                                               "external-chart-combination-legal-execution"), 1):
                    actual = session.run(label, ["fixture"], 1)
                    self.assertEqual(actual, raw)
                    self.assertEqual(contracts.contract_counts(actual), expected)
                    self.assertEqual(session.check.call_count, 2 * index)
                    self.assertEqual(session.report["lanes"][-1]["output"], raw)
                    self.assertEqual(json.loads((root / "progress.json").read_text())["lanes"][-1]["output"], raw)
                for label in ("ordinary-control", "external-adaptive-legal-execution-extra",
                              "external-chart-legal-execution-extra",
                              "external-chart-combination-legal-execution-extra"):
                    with self.subTest(label=label):
                        self.assertEqual(session.run(label, ["fixture"], 1), raw.strip())
            for exit_code, failure in ((1, None), (0, "deadline")):
                rejected = {**success, "exit_code": exit_code, "failure": failure}
                with self.subTest(exit_code=exit_code, failure=failure), \
                        patch.object(MODULE.RUNTIME, "run", return_value=rejected):
                    with self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                        session.run("external-adaptive-legal-execution", ["fixture"], 1)
                    with self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                        session.run("external-chart-legal-execution", ["fixture"], 1)
                    with self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                        session.run("external-chart-combination-legal-execution", ["fixture"], 1)

    def test_mutation_listing_admits_only_exact_warning_and_complete_strict_json(self):
        warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        listing = '[{"name":"one","file":"selected","package":"fixture"}]'
        expected = [{"name": "one", "file": "selected", "package": "fixture"}]
        self.assertEqual(MODULE.parse_mutation_listing(listing), (expected, None))
        self.assertEqual(MODULE.parse_mutation_listing(warning + listing + "\n"), (expected, warning))
        warning_ten = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
        self.assertEqual(MODULE.parse_mutation_listing(warning_ten + listing, expected_jobs=10),
                         (expected, warning_ten))
        self.assertEqual(MODULE.parse_mutation_listing(listing, expected_jobs=2), (expected, None))
        for output, workers in ((warning, 10), (warning_ten, 12), (warning_ten, 2), (warning, 8)):
            with self.subTest(workers=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(output + listing, expected_jobs=workers)
        for workers in (True, False, 0, 13, 10.0, "10", None):
            with self.subTest(workers=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(listing, expected_jobs=workers)
        invalid = [
            "unexpected diagnostic\n" + listing,
            " " + warning + listing,
            warning.replace("--jobs=12", "--jobs=13") + listing,
            warning.replace(" WARN", "WARN") + listing,
            warning.rstrip("\n") + listing,
            warning + warning + listing,
            warning + "warning: another message\n" + listing,
            warning + listing + "\ntrailing diagnostic",
            listing + " []",
            listing[:-1],
            '{"name":"not a list"}',
            '[{"name":"first","name":"second"}]',
            '[{"value":NaN}]',
            '[{"value":Infinity}]',
            '[{"value":1e999}]',
            "",
            None,
        ]
        for output in invalid:
            with self.subTest(output=output), self.assertRaises(MODULE.Incomplete):
                MODULE.parse_mutation_listing(output)

    def test_ten_worker_listing_retains_native_warning_and_reaches_campaign(self):
        warning = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            report = {}
            calls = []
            listed = warning + json.dumps([{"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}])
            def run(label, command, _timeout, **options):
                calls.append(label)
                self.assertEqual(command[command.index("--jobs") + 1], "10")
                if label == "mutation-enumeration:era":
                    return listed
                self.assertEqual(label, "mutation-campaign:era")
                self.assertEqual(options, {"mutation_subject": "era"})
                raise RuntimeError("controlled stop after admitted ten-worker listing")
            policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                      "mutation_jobs": 10, "mutation_process_limit": 512}
            with self.assertRaisesRegex(RuntimeError, "controlled stop after admitted ten-worker listing"):
                MODULE.run_analysis_lanes(SCOPE, {"era": dict(FILES)["era"]}, policy,
                                          work, work / "tools", run, report, "mutation")
            self.assertEqual(calls, ["mutation-enumeration:era", "mutation-campaign:era"])
            self.assertEqual(report["mutation_enumeration"]["era"], {"raw_output": listed, "warning": warning})

    def test_failed_mutation_listing_retains_raw_warning_and_starts_no_campaign(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary); tools = work / "tools"; tools.mkdir()
            warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
            raw = warning + "["
            calls, report = [], {}
            def run(label, _command, _timeout, **_options):
                calls.append(label)
                return raw
            with self.assertRaises(MODULE.Incomplete):
                MODULE.run_analysis_lanes(SCOPE, dict(FILES[:1]),
                    {"mutation_wall_seconds": 1200, "maximum_mutants": 512},
                    work, tools, run, report, "mutation")
            self.assertEqual(calls, ["mutation-enumeration:coordinate"])
            self.assertEqual(report["mutation_enumeration"]["coordinate"],
                             {"raw_output": raw, "warning": warning})

    def test_inventory_only_lists_exact_owners_without_baselines_or_campaigns(self):
        """A larger discovery reservation reports, but never admits, a campaign."""
        subjects = {"era": dict(FILES)["era"], "clock": dict(FILES)["clock"]}
        mutants = {name: {"file": filename, "name": name, "package": "time-domain"}
                   for name, filename in subjects.items()}
        warning = (" WARN --jobs=10 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        report, calls = {}, []
        tools = Path("/sealed/tools")
        def run(label, command, timeout, **options):
            calls.append(label)
            name = label.removeprefix("mutation-enumeration:")
            self.assertIn(name, subjects)
            self.assertEqual(command, [str(tools / "cargo-mutants"), "mutants", "--package", "time-domain",
                "--file", subjects[name], "--test-workspace", "false", "--cap-lints", "false", "--jobs", "10",
                "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
                "--list", "--json"])
            self.assertEqual(options, {})
            self.assertGreater(timeout, 0)
            self.assertLessEqual(timeout, 120)
            return (warning if name == "era" else "") + json.dumps([mutants[name]])
        policy = {"maximum_mutants": 1, "mutation_jobs": 10,
                  "mutation_process_limit": 512, "mutation_message_format": "json"}
        found = MODULE.enumerate_mutation_inventories(SCOPE, subjects, policy, tools, run, report,
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=2, maximum_total=4)
        self.assertEqual(calls, ["mutation-enumeration:era", "mutation-enumeration:clock"])
        self.assertEqual(found.inventories, {name: {name: mutant} for name, mutant in mutants.items()})
        self.assertEqual(found.total, 2)
        self.assertTrue(found.campaign_budget_exceeded)
        self.assertEqual(report["mutation_enumerated_total"], 2)
        self.assertTrue(report["mutation_campaign_budget_exceeded"])
        self.assertEqual(found.observations, report["mutation_enumeration"])
        self.assertEqual(found.observations["era"], {"raw_output": warning + json.dumps([mutants["era"]]),
                                                  "warning": warning})
        self.assertIsNone(found.observations["clock"]["warning"])
        self.assertTrue(all("--baseline" not in command and "--output" not in command
                            for command in found.commands.values()))
        self.assertNotIn("mutation", report)
        self.assertNotIn("mutation_reviewed_inventories", report)

    def test_inventory_only_requires_exact_scope_and_explicit_finite_allowances(self):
        subjects = {"era": dict(FILES)["era"]}
        policy = {"maximum_mutants": 2}
        valid = {"deadline": MODULE.time.monotonic() + 60, "maximum_per_file": 2, "maximum_total": 2}
        for field, values in {
            "maximum_per_file": (True, False, None, "2", 2.0, 0, -1, 513),
            "maximum_total": (True, None, "2", 2.0, 0, -1, 3),
            "deadline": (True, None, "later", float("inf"), float("nan"), 10**500, 0),
        }.items():
            for value in values:
                with self.subTest(field=field, value=value):
                    run = unittest.mock.Mock()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.enumerate_mutation_inventories(SCOPE, subjects, policy, Path("/tools"), run, {},
                                                              **{**valid, field: value})
                    run.assert_not_called()
        for selected in ({}, {"unknown": dict(FILES)["era"]}, {"unknown": None},
                         {"era": dict(FILES)["clock"]}):
            with self.subTest(subjects=selected):
                run = unittest.mock.Mock()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, selected, policy, Path("/tools"), run, {}, **valid)
                run.assert_not_called()
        for value in (True, None, "2", 2.0, 0, -1, 513):
            with self.subTest(campaign_cap=value):
                run = unittest.mock.Mock()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": value},
                                                          Path("/tools"), run, {}, **valid)
                run.assert_not_called()

    def test_inventory_only_retains_rejected_native_listings(self):
        mutant = {"name": "one", "file": dict(FILES)["era"], "package": "time-domain"}
        warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                   "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                   "are usually safe\n")
        invalid = [("[", 1), ("{}", 1), ("[]", 1), (json.dumps([mutant, mutant]), 2),
                   (json.dumps([{**mutant, "file": dict(FILES)["clock"]}]), 1),
                   (json.dumps([{**mutant, "package": "other"}]), 1),
                   (json.dumps([mutant, {**mutant, "name": "two"}]), 1),
                   (warning + json.dumps([mutant]), 1)]
        for raw, allowance in invalid:
            with self.subTest(raw=raw):
                report = {"mutation_enumerated_total": 1, "mutation_campaign_budget_exceeded": False}
                run = unittest.mock.Mock(return_value=raw)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.enumerate_mutation_inventories(SCOPE, {"era": dict(FILES)["era"]},
                        {"maximum_mutants": 512, "mutation_jobs": 10}, Path("/tools"), run, report,
                        deadline=MODULE.time.monotonic() + 60,
                        maximum_per_file=allowance, maximum_total=allowance)
                self.assertEqual(run.call_count, 1)
                self.assertEqual(run.call_args.args[0], "mutation-enumeration:era")
                self.assertEqual(run.call_args.args[1][-2:], ["--list", "--json"])
                self.assertEqual(report["mutation_enumeration"]["era"]["raw_output"], raw)
                self.assertNotIn("mutation", report)
                self.assertNotIn("mutation_enumerated_total", report)
                self.assertNotIn("mutation_campaign_budget_exceeded", report)

    def test_inventory_only_stops_on_its_separate_aggregate_listing_allowance(self):
        subjects = dict(FILES[:2])
        calls, report = [], {}
        def run(label, command, _timeout):
            calls.append(label)
            name = label.split(":")[1]
            self.assertEqual(command[-2:], ["--list", "--json"])
            return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 512},
                Path("/tools"), run, report, deadline=MODULE.time.monotonic() + 60,
                maximum_per_file=1, maximum_total=1)
        self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
        self.assertEqual(set(report["mutation_enumeration"]), set(subjects))
        self.assertNotIn("mutation", report)

    def test_inventory_only_consumes_one_unchanged_deadline(self):
        subjects = dict(FILES[:2])
        for clocks, expected_timeouts, exhausted in [([90.25, 98.1], [9, 1], False),
                                                      ([90.25, 100.1], [9], True)]:
            with self.subTest(clocks=clocks):
                timeouts, calls, report = [], [], {}
                def run(label, command, timeout):
                    calls.append(label)
                    timeouts.append(timeout)
                    self.assertEqual(command[-2:], ["--list", "--json"])
                    name = label.split(":")[1]
                    return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
                with patch.object(MODULE.time, "monotonic", side_effect=clocks):
                    if exhausted:
                        with self.assertRaises(MODULE.Incomplete):
                            MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 2},
                                Path("/tools"), run, report, deadline=100.0, maximum_per_file=1, maximum_total=2)
                    else:
                        found = MODULE.enumerate_mutation_inventories(SCOPE, subjects, {"maximum_mutants": 2},
                            Path("/tools"), run, report, deadline=100.0, maximum_per_file=1, maximum_total=2)
                        self.assertEqual(found.total, 2)
                        self.assertFalse(found.campaign_budget_exceeded)
                self.assertEqual(timeouts, expected_timeouts)
                self.assertEqual(calls, ["mutation-enumeration:" + name
                                         for name in list(subjects)[:len(expected_timeouts)]])
                if exhausted:
                    self.assertNotIn("mutation_enumerated_total", report)

    def test_campaign_lanes_keep_the_aggregate_gate_before_every_campaign(self):
        subjects = dict(FILES[:2])
        for lane in ("mutation", "all"):
            with self.subTest(lane=lane), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                calls, report = [], {}
                def run(label, command, _timeout, **_options):
                    calls.append(label)
                    self.assertNotIn("--baseline", command)
                    if label == "coverage":
                        (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                            {"filename": "/work/source/" + filename,
                             "summary": {"lines": {"count": 10, "covered": 9}}}
                            for filename in subjects.values()]}]}))
                        return "test result: ok. 1 passed; 0 failed; 0 ignored;"
                    name = label.removeprefix("mutation-enumeration:")
                    self.assertIn(name, subjects)
                    return json.dumps([{"name": name, "file": subjects[name], "package": "time-domain"}])
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.run_analysis_lanes(SCOPE, subjects,
                        {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200, "maximum_mutants": 1},
                        work, work / "tools", run, report, lane)
                self.assertEqual(calls, (["coverage"] if lane == "all" else [])
                                 + ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
                self.assertNotIn("mutation", report)

    def test_scope_is_immutable(self):
        with self.assertRaises(AttributeError):
            SCOPE.package = "other"

    def test_each_file_requires_its_actual_nonempty_line_denominator(self):
        for _, filename in FILES:
            full = "/work/source/" + filename
            report = {"data": [{"files": [{"filename": full, "summary": {
                "lines": {"count": 10, "covered": 9}}}]}]}
            self.assertEqual(MODULE.coverage_counts(SCOPE, report, full)["line_percent"], 90)
            for count, covered in [(0, 0), (10, 8), (True, 1), (10, 11)]:
                bad = {"data": [{"files": [{"filename": full, "summary": {
                    "lines": {"count": count, "covered": covered}}}]}]}
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.coverage_counts(SCOPE, bad, full)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.coverage_counts(SCOPE, report, "/work/source/crates/other.rs")

    def test_native_coverage_filter_keeps_all_six_owners_and_the_file_threshold(self):
        """The observed upstream 3/6 file must not enter the native owner gate."""
        subjects = dict(CORE_FILES)
        scope = MODULE.Scope("time-core", tuple(subjects.items()), "policy/time-core-analysis.json")
        sysroot = ("/home/david_osipov/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/"
                   "lib/rustlib/src/rust/library/std/src/sys/thread_local/native/mod.rs")
        test_source = "/work/source/crates/time-core/tests/selection.rs"
        files = [{"filename": "/work/source/" + filename,
                  "summary": {"lines": {"count": 10, "covered": 9}}}
                 for filename in subjects.values()]
        files.extend([{"filename": sysroot, "summary": {"lines": {"count": 6, "covered": 3}}},
                      {"filename": test_source, "summary": {"lines": {"count": 10, "covered": 0}}}])
        with tempfile.TemporaryDirectory() as temporary:
            work, report, calls = Path(temporary), {}, []
            def run(label, command, timeout):
                calls.append(label)
                self.assertEqual((label, timeout), ("coverage", 300))
                self.assertTrue({"--frozen", "--lib", "--tests", "--json"}.issubset(command))
                self.assertEqual(command[command.index("--failure-mode") + 1], "any")
                self.assertEqual(command[command.index("--fail-under-file-lines") + 1], "90")
                self.assertNotIn("--disable-default-ignore", command)
                self.assertEqual(command.count("--ignore-filename-regex"), 1)
                excluded = re.compile(command[command.index("--ignore-filename-regex") + 1])
                self.assertIsNotNone(excluded.search(sysroot))
                self.assertIsNotNone(excluded.search(test_source))
                self.assertIsNone(excluded.search("/work/source/crates/time-core/src/thread_local/native/mod.rs"))
                for filename in subjects.values():
                    self.assertIsNone(excluded.search("/work/source/" + filename))
                selected = [item for item in files if not excluded.search(item["filename"])]
                self.assertEqual(len(selected), 6)
                (work / "coverage.json").write_text(json.dumps({"data": [{"files": selected}]}))
                return "test result: ok. 94 passed; 0 failed; 0 ignored;"
            MODULE.run_analysis_lanes(scope, subjects, {"coverage_wall_seconds": 300},
                                      work, work / "tools", run, report, "coverage")
            self.assertEqual(calls, ["coverage"])
            self.assertEqual(set(report["coverage"]), set(subjects))
            self.assertTrue(all(value["line_percent"] == 90 and value["tests_executed"] == 94
                                for value in report["coverage"].values()))

    def test_coverage_test_threads_are_opt_in_and_forwarded_after_the_separator(self):
        """Honor selected Cargo jobs independently of the opt-in libtest limit."""
        core = MODULE.Scope("time-core", CORE_FILES, "policy/time-core-analysis.json")
        for scope, subjects, override, jobs, suffix in (
                (SCOPE, dict(FILES), {}, "12", []),
                (core, dict(CORE_FILES), {}, "12", []),
                (core, dict(CORE_FILES), {"coverage_test_threads": 1}, "12", ["--", "--test-threads=1"]),
                (core, dict(CORE_FILES), {"jobs": 6, "coverage_test_threads": 1}, "6", ["--", "--test-threads=1"])):
            with self.subTest(package=scope.package, override=override), tempfile.TemporaryDirectory() as temporary:
                work, report, calls = Path(temporary), {}, []
                def run(label, command, timeout):
                    calls.append(label)
                    self.assertEqual((label, timeout), ("coverage", 300))
                    self.assertEqual(command, [
                        str(work / "tools" / "cargo-llvm-cov"), "llvm-cov", "--package", scope.package,
                        "--frozen", "--jobs", jobs, "--lib", "--tests", "--json",
                        "--output-path", "/work/coverage.json", "--failure-mode", "any",
                        "--ignore-filename-regex", "(^|/)tests/|/rustlib/src/rust/library/",
                        "--fail-under-file-lines", "90", *suffix])
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename,
                         "summary": {"lines": {"count": 10, "covered": 9}}}
                        for filename in subjects.values()]}]}))
                    return "test result: ok. 8 passed; 0 failed; 0 ignored;"
                MODULE.run_analysis_lanes(scope, subjects, {"coverage_wall_seconds": 300, **override},
                                          work, work / "tools", run, report, "coverage")
                self.assertEqual(calls, ["coverage"])
                self.assertEqual(set(report["coverage"]), set(subjects))
                self.assertTrue(all(row["tests_executed"] == 8 for row in report["coverage"].values()))

    def test_invalid_coverage_thread_overrides_stop_before_any_lane_effect(self):
        for lane in ("coverage", "mutation", "all"):
            for value in (None, True, False, 0, 2, 12, -1, "1", 1.0, [], {}):
                with self.subTest(lane=lane, value=value), tempfile.TemporaryDirectory() as temporary:
                    work, report = Path(temporary), {}
                    run = unittest.mock.Mock()
                    policy = {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
                              "maximum_mutants": 512, "coverage_test_threads": value}
                    with patch.object(MODULE, "admit_mutation_equivalences") as admit:
                        with self.assertRaisesRegex(MODULE.Incomplete, "Coverage test-thread override"):
                            MODULE.run_analysis_lanes(SCOPE, dict(FILES), policy,
                                                      work, work / "tools", run, report, lane)
                        admit.assert_not_called()
                    run.assert_not_called()
                    self.assertFalse((work / "mutation").exists())
                    self.assertEqual(report, {})

    def test_coverage_scope_repair_cannot_hide_missing_duplicate_or_undercovered_owner(self):
        """Good neighbors and an upstream file cannot replace a declared owner."""
        subjects = dict(CORE_FILES)
        scope = MODULE.Scope("time-core", CORE_FILES, "policy/time-core-analysis.json")
        for selected_name, filename in subjects.items():
            for fault in ("missing", "duplicate", "below-threshold"):
                with self.subTest(owner=selected_name, fault=fault), tempfile.TemporaryDirectory() as temporary:
                    work, report = Path(temporary), {}
                    files = [{"filename": "/work/source/" + owner,
                              "summary": {"lines": {"count": 1000, "covered": 1000}}}
                             for owner in subjects.values() if owner != filename]
                    selected = {"filename": "/work/source/" + filename,
                                "summary": {"lines": {"count": 10, "covered": 8 if fault == "below-threshold" else 9}}}
                    if fault != "missing":
                        files.append(selected)
                    if fault == "duplicate":
                        files.append(selected)
                    files.append({"filename": "/rustlib/src/rust/library/std/src/lib.rs",
                                  "summary": {"lines": {"count": 10000, "covered": 10000}}})
                    def run(label, _command, _timeout):
                        self.assertEqual(label, "coverage")
                        (work / "coverage.json").write_text(json.dumps({"data": [{"files": files}]}))
                        return "test result: ok. 94 passed; 0 failed; 0 ignored;"
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.run_analysis_lanes(scope, subjects, {"coverage_wall_seconds": 300},
                                                  work, work / "tools", run, report, "coverage")
                    self.assertNotIn("coverage", report)

    def test_viable_mutants_require_actual_tests_and_exact_full_inventory(self):
        filename = FILES[0][1]
        items = [{"name": f"fault-{index}", "file": filename, "package": "time-domain"}
                 for index in range(2)]
        frozen = MODULE.mutation_inventory(SCOPE, items, 512, filename)
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 2, "outcomes": [
            {"scenario": "Baseline", "summary": "Success"},
            {"scenario": {"Mutant": items[0]}, "summary": "CaughtMutant", "phase_results": [{"phase": "Test"}]},
            {"scenario": {"Mutant": items[1]}, "summary": "Unviable"}]}
        report = complete_report(report)
        actual = MODULE.mutation_counts(SCOPE, report, frozen, filename)
        self.assertEqual((actual["viable"], actual["killed"], actual["unviable"]), (1, 1, 1))
        for replacement in ["MissedMutant", "Timeout"]:
            bad = json.loads(json.dumps(report))
            bad["outcomes"][2].update(summary=replacement, phase_results=[{"phase": "Test"}])
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, bad, frozen, filename)
        for bad_items in [[], [items[0], items[0]], [{**items[0], "package": "other"}]]:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_inventory(SCOPE, bad_items, 512, filename)
        bad = json.loads(json.dumps(report)); bad["outcomes"].pop()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, bad, frozen, filename)

    def test_reviewed_inventories_require_full_scope_shapes_and_exact_normalized_hash(self):
        filename = FILES[0][1]
        first = {"name": "one", "file": filename, "package": "time-domain",
                 "replacement": "x", "nested": {"diff": "retained"}}
        second = {"name": "two", "file": filename, "package": "time-domain", "replacement": "y"}
        normalized = {"one": first, "two": second}
        digest = hashlib.sha256(json.dumps(normalized, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        specs = {name: {"count": 1, "sha256": "0" * 64} for name, _ in FILES}
        specs["coordinate"] = {"count": 2, "sha256": digest}
        selected = {"coordinate": {"two": {**second, "diff": "rendered"}, "one": first}}
        policy = {"maximum_mutants": 512, "mutation_inventories": specs}
        admitted = MODULE.admit_reviewed_mutation_inventories(SCOPE, policy, selected)
        self.assertEqual(admitted, {"coordinate": {"count": 2, "sha256": digest}})
        self.assertIsNone(MODULE.admit_reviewed_mutation_inventories(SCOPE, {"maximum_mutants": 512}, selected))
        altered = {"coordinate": {"one": {**first, "nested": {"diff": "changed"}}, "two": second}}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_reviewed_mutation_inventories(SCOPE, policy, altered)
        for replacement in (None, {}, {**specs, "extra": specs["era"]},
                            {name: spec for name, spec in specs.items() if name != "era"}):
            with self.subTest(mapping=replacement), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": replacement}, selected)
        for field, value in (("count", True), ("count", 0), ("count", 513), ("count", 3),
                             ("sha256", "f" * 63), ("sha256", "A" * 64), ("sha256", "0" * 64)):
            bad = {name: dict(spec) for name, spec in specs.items()}
            bad["coordinate"][field] = value
            with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": bad}, selected)
        for spec in ({"count": 1}, {"count": 1, "sha256": "0" * 64, "extra": 1}):
            bad = {name: dict(item) for name, item in specs.items()}; bad["era"] = spec
            with self.subTest(unselected=spec), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_reviewed_mutation_inventories(
                    SCOPE, {"maximum_mutants": 512, "mutation_inventories": bad}, selected)

    def test_reviewed_inventory_mismatch_stops_all_campaigns_after_all_listings(self):
        subjects = dict(FILES[:2])
        mutants = {name: {"name": name, "file": filename, "package": "time-domain", "replacement": "x"}
                   for name, filename in subjects.items()}
        def digest_for(mutant):
            payload = json.dumps({mutant["name"]: mutant}, sort_keys=True, separators=(",", ":")).encode()
            return hashlib.sha256(payload).hexdigest()
        specs = {name: {"count": 1, "sha256": "0" * 64} for name, _ in FILES}
        for name, mutant in mutants.items():
            specs[name] = {"count": 1, "sha256": digest_for(mutant)}
        for mismatch in (False, True):
            with self.subTest(mismatch=mismatch), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); tools = work / "tools"; tools.mkdir()
                reviewed = {name: dict(spec) for name, spec in specs.items()}
                if mismatch:
                    reviewed["era"]["sha256"] = "0" * 64
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512,
                          "mutation_inventories": reviewed}
                calls, report = [], {}
                def run(label, _command, _timeout, **_options):
                    calls.append(label)
                    name = label.split(":")[1]
                    mutant = mutants[name]
                    if label.startswith("mutation-enumeration:"):
                        return json.dumps([{**mutant, "diff": "ignored rendered diff"}])
                    directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                    (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                    (directory / "outcomes.json").write_text(json.dumps(complete_report({
                        "cargo_mutants_version": "27.1.0", "total_mutants": 1,
                        "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                     {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})))
                    return {"exit_code": 0, "failure": None, "output": "completed"}
                if mismatch:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                    self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era"])
                else:
                    MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                    self.assertEqual(calls, ["mutation-enumeration:coordinate", "mutation-enumeration:era",
                                             "mutation-campaign:coordinate", "mutation-campaign:era"])
                    self.assertEqual(report["mutation_reviewed_inventories"],
                                     {name: specs[name] for name in subjects})

    def test_caught_summary_cannot_borrow_a_successful_test_result(self):
        filename = FILES[0][1]
        mutant = {"name": "phase-status", "file": filename, "package": "time-domain"}
        phases = phase_records()
        report = {"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                  "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases},
                               {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": phases}]}
        report = complete_report(report)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, {mutant["name"]: mutant}, filename)

    def test_complete_missed_file_does_not_hide_later_campaign(self):
        for first_summary in ("MissedMutant", "Unviable", "CaughtMutant"):
            with self.subTest(first_summary=first_summary), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary); tools = work / "tools"; tools.mkdir()
                subjects = dict(FILES[:2]); report = {}; observed = []
                policy = {"mutation_wall_seconds": 1200, "maximum_mutants": 512}
                def run(label, command, timeout, **options):
                    name = label.split(":")[1]; filename = subjects[name]
                    mutant = {"name": name, "file": filename, "package": "time-domain"}
                    if label.startswith("mutation-enumeration:"):
                        return json.dumps([mutant])
                    observed.append(name)
                    summary = first_summary if name == "coordinate" else "CaughtMutant"
                    directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                    (directory / "baseline.log").write_text("test result: ok. 1 passed; 0 failed; 0 ignored;")
                    (directory / "outcomes.json").write_text(json.dumps(complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                        "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phase_records(), "log_path": "baseline.log"},
                                     {"scenario": {"Mutant": mutant}, "summary": summary, "phase_results": phase_records(summary)}]})))
                    return {"exit_code": 2 if summary == "MissedMutant" else 0, "failure": None, "output": "observed"}
                if first_summary == "CaughtMutant":
                    MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                    self.assertTrue(all(row["gate_passed"] for row in report["mutation"].values()))
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, "mutation")
                self.assertEqual(observed, ["coordinate", "era"])
                self.assertEqual(set(report["mutation"]), set(subjects))
                if first_summary == "Unviable":
                    self.assertIs(report["mutation"]["coordinate"]["gate_passed"], False)
                    self.assertIsNone(report["mutation"]["coordinate"]["adjusted_viable_killed_percent"])
                    self.assertIs(report["mutation"]["era"]["gate_passed"], True)

    def test_all_unviable_report_is_unscored_only_when_explicitly_deferred(self):
        filename = FILES[0][1]
        mutant = {"name": "unviable-control", "file": filename, "package": "time-domain"}
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
            "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                         {"scenario": {"Mutant": mutant}, "summary": "Unviable"}]})
        frozen = MODULE.mutation_inventory(SCOPE, [mutant], 512, filename)
        for deferred in (False, 0, 1, None, "true"):
            with self.subTest(deferred=deferred), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, defer_gate=deferred)
        actual = MODULE.mutation_counts(SCOPE, report, frozen, filename, defer_gate=True)
        self.assertEqual((actual["generated"], actual["viable"], actual["killed"], actual["survived"],
                          actual["unviable"], actual["excluded"]), (1, 0, 0, 0, 1, 0))
        self.assertEqual((actual["raw_denominator"], actual["adjusted_denominator"]), (0, 0))
        self.assertIs(actual["gate_passed"], False)
        for field in ("raw_viable_killed_percent", "viable_killed_percent", "adjusted_viable_killed_percent"):
            self.assertIsNone(actual[field])
        edits = [lambda p: p.update(unviable=0), lambda p: p.update(total_mutants=True),
            lambda p: p["outcomes"].pop(), lambda p: p["outcomes"][0].update(summary="Unviable"),
            lambda p: p["outcomes"][1].update(summary="Timeout"),
            lambda p: p["outcomes"][1]["phase_results"][0].update(process_status="Success"),
            lambda p: p["outcomes"][1]["phase_results"][0].update(duration=float("nan")),
            lambda p: p["outcomes"][1]["scenario"]["Mutant"].update(unreviewed=True)]
        for edit in edits:
            bad = copy.deepcopy(report); edit(bad)
            with self.subTest(edit=edit), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, bad, frozen, filename, defer_gate=True)
        for native in (2, 3, -9, True):
            with self.subTest(native=native), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=native, defer_gate=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, {}, filename, defer_gate=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, frozen, filename,
                                   equivalences=[{"mutant": mutant}], defer_gate=True)
        missed = copy.deepcopy(report)
        missed["outcomes"][1].update(summary="MissedMutant", phase_results=phase_records("MissedMutant"))
        missed = complete_report(missed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, missed, frozen, filename, native_exit_code=2,
                                   equivalences=[{"mutant": mutant}], defer_gate=True)

    def test_equivalence_admission_binds_source_candidate_proof_and_exact_review_section(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); filename = FILES[1][1]
            source = root / filename; source.parent.mkdir(parents=True); source.write_text("frozen numerical owner\n")
            proof = root / "tasks/proof.md"; proof.parent.mkdir(); proof.write_text("## TD-E-001 — proof\nIndependent rationale.\n")
            digest = lambda data: hashlib.sha256(data).hexdigest()
            identity = {filename: digest(source.read_bytes()), "other-construction-owner": "a" * 64}
            candidate = digest(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode())
            mutant = {"name": "complete-era-fault", "file": filename, "package": "time-domain",
                      "function": {"function_name": "resolve", "return_type": "-> Result<_, _>",
                                   "span": {"start": {"line": 1, "column": 1}, "end": {"line": 5, "column": 2}}},
                      "span": {"start": {"line": 3, "column": 1}, "end": {"line": 3, "column": 2}},
                      "replacement": "^", "genre": "BinaryOperator"}
            entry = {"id": "TD-E-001", "candidate_sha256": candidate, "source_sha256": identity[filename],
                     "mutant": mutant, "requirement": "CHECK-6/7/25", "rationale": "Disjoint bit fields preserve every input.",
                     "proof": {"path": "tasks/proof.md", "sha256": digest(proof.read_bytes()), "section": "TD-E-001",
                               "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
            policy = {"mutation_equivalences": [entry], "maximum_mutants": 512}
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.admit_mutation_equivalences(root, SCOPE, policy), [entry])
                for change in ("source", "candidate", "proof", "section", "review", "file", "identity", "duplicate", "unknown",
                               "function_empty", "return_none", "return_bool", "return_list", "return_missing", "return_overlong"):
                    bad = json.loads(json.dumps(policy)); case = bad["mutation_equivalences"][0]
                    if change == "source": case["source_sha256"] = "f" * 64
                    if change == "candidate": case["candidate_sha256"] = "f" * 64
                    if change == "proof": case["proof"]["sha256"] = "f" * 64
                    if change == "section": case["id"] = case["proof"]["section"] = "TD-E-002"
                    if change == "review": case["proof"]["verdict"] = "probably-safe"
                    if change == "file": case["mutant"]["file"] = "crates/other/src/era.rs"
                    if change == "identity": case["mutant"].pop("span")
                    if change == "duplicate": bad["mutation_equivalences"].append(dict(case))
                    if change == "unknown": case["skip_function"] = "resolve"
                    if change == "function_empty": case["mutant"]["function"]["function_name"] = ""
                    if change == "return_none": case["mutant"]["function"]["return_type"] = None
                    if change == "return_bool": case["mutant"]["function"]["return_type"] = False
                    if change == "return_list": case["mutant"]["function"]["return_type"] = []
                    if change == "return_missing": case["mutant"]["function"].pop("return_type")
                    if change == "return_overlong": case["mutant"]["function"]["return_type"] = "x" * 4097
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_mutation_equivalences(root, SCOPE, bad)
                source.write_text("changed numerical owner\n")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                source.write_text("frozen numerical owner\n")
                proof.write_text("## TD-E-001 — changed proof\n")
                with self.assertRaisesRegex(MODULE.Incomplete, "proof bytes"):
                    MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                proof.write_text("## TD-E-001 — proof\nIndependent rationale.\n")
                with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, {**identity, "other-construction-owner": "b" * 64})):
                    with self.assertRaisesRegex(MODULE.Incomplete, "complete candidate"):
                        MODULE.admit_mutation_equivalences(root, SCOPE, policy)
            self.assertEqual(MODULE.admit_mutation_equivalences(root, SCOPE, {}), [])

    def test_exact_unary_deletion_equivalence_preserves_empty_replacement(self):
        # Literal cargo-mutants 27.1.0 descriptor from the actual rounding campaign.
        mutant = {"name": "crates/time-domain/src/rounding.rs:59:26: delete - in round",
                  "package": "time-domain", "file": "crates/time-domain/src/rounding.rs",
                  "function": {"function_name": "round", "return_type": "-> Result<(i64, i128), DomainError>",
                               "span": {"start": {"line": 26, "column": 1}, "end": {"line": 64, "column": 2}}},
                  "span": {"start": {"line": 59, "column": 26}, "end": {"line": 59, "column": 27}},
                  "replacement": "", "genre": "UnaryOperator"}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); filename = mutant["file"]
            source = root / filename; source.parent.mkdir(parents=True); source.write_text("frozen rounding owner\n")
            proof = root / "tasks/proof.md"; proof.parent.mkdir(); proof.write_text("## TD-E-006 — proof\nIndependent rationale.\n")
            digest = lambda data: hashlib.sha256(data).hexdigest()
            identity = {filename: digest(source.read_bytes())}
            candidate = digest(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode())
            entry = {"id": "TD-E-006", "candidate_sha256": candidate, "source_sha256": identity[filename],
                     "mutant": mutant, "requirement": "CHECK-6/7/25", "rationale": "Exact independently reviewed deletion.",
                     "proof": {"path": "tasks/proof.md", "sha256": digest(proof.read_bytes()), "section": "TD-E-006",
                               "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
            policy = {"mutation_equivalences": [entry], "maximum_mutants": 512}
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                admitted = MODULE.admit_mutation_equivalences(root, SCOPE, policy)
                self.assertEqual(admitted, [entry])
                self.assertEqual(admitted[0]["mutant"]["replacement"], "")
                for key, value in (("genre", "BinaryOperator"), ("genre", "FnValue"), ("genre", ""),
                                   ("genre", None), ("genre", False), ("replacement", None),
                                   ("replacement", False), ("replacement", []),
                                   ("file", "crates/other/src/rounding.rs")):
                    bad = json.loads(json.dumps(policy)); bad["mutation_equivalences"][0]["mutant"][key] = value
                    with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.admit_mutation_equivalences(root, SCOPE, bad)
            caught = {**mutant, "name": "caught rounding neighbor", "replacement": "-"}
            report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 2,
                "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                             {"scenario": {"Mutant": caught}, "summary": "CaughtMutant"},
                             {"scenario": {"Mutant": mutant}, "summary": "MissedMutant"}]})
            frozen = MODULE.mutation_inventory(SCOPE, [caught, mutant], 512, filename)
            result = MODULE.mutation_counts(SCOPE, report, frozen, filename,
                                            native_exit_code=2, equivalences=admitted)
            self.assertTrue(result["gate_passed"])
            self.assertEqual(result["excluded_cases"][0]["mutant"], mutant)
            for key, value in (("replacement", "-"), ("genre", "BinaryOperator")):
                bad = json.loads(json.dumps(admitted)); bad[0]["mutant"][key] = value
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=bad)

    def test_exact_missed_equivalences_keep_raw_and_adjusted_denominators_distinct(self):
        filename = FILES[1][1]
        mutants = [{"name": f"era-{index}", "file": filename, "package": "time-domain", "replacement": str(index)}
                   for index in range(25)]
        outcomes = [{"scenario": "Baseline", "summary": "Success"}]
        for index, mutant in enumerate(mutants):
            summary = "CaughtMutant" if index < 17 else "MissedMutant" if index < 19 else "Unviable"
            outcomes.append({"scenario": {"Mutant": mutant}, "summary": summary})
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 25, "outcomes": outcomes})
        frozen = MODULE.mutation_inventory(SCOPE, mutants, 512, filename)
        equivalents = [{"id": f"TD-E-00{index - 16}", "mutant": mutants[index]} for index in (17, 18)]
        actual = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=equivalents)
        self.assertEqual((actual["generated"], actual["killed"], actual["survived"], actual["unviable"], actual["excluded"]),
                         (25, 17, 2, 6, 2))
        self.assertEqual((actual["raw_denominator"], actual["adjusted_denominator"], actual["native_exit_code"]), (19, 17, 2))
        self.assertAlmostEqual(actual["raw_viable_killed_percent"], 89.47368421052632)
        self.assertEqual(actual["adjusted_viable_killed_percent"], 100)
        self.assertTrue(actual["gate_passed"])
        self.assertEqual(actual["excluded_cases"], equivalents)
        for bad in (equivalents[:1], equivalents + [equivalents[0]],
                    [{"mutant": {**mutants[17], "replacement": "changed"}}], [{"mutant": mutants[0]}],
                    [{"mutant": {**mutants[17], "file": FILES[0][1]}}]):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, equivalences=bad)
        unresolved = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, defer_gate=True)
        self.assertFalse(unresolved["gate_passed"])
        self.assertEqual(unresolved["unresolved_survivors"], ["era-17", "era-18"])

    def test_passing_raw_percentage_does_not_discharge_an_unknown_survivor(self):
        filename = FILES[0][1]
        mutants = [{"name": f"fault-{index}", "file": filename, "package": "time-domain"} for index in range(21)]
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 21,
            "outcomes": [{"scenario": "Baseline", "summary": "Success"}] + [
                {"scenario": {"Mutant": mutant}, "summary": "MissedMutant" if index == 20 else "CaughtMutant"}
                for index, mutant in enumerate(mutants)]})
        frozen = MODULE.mutation_inventory(SCOPE, mutants, 512, filename)
        result = MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2, defer_gate=True)
        self.assertGreater(result["raw_viable_killed_percent"], 95)
        self.assertFalse(result["gate_passed"])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=2)

    def test_native_exit_counter_phase_baseline_and_inventory_mismatches_reject(self):
        filename = FILES[0][1]; mutant = {"name": "one", "file": filename, "package": "time-domain"}
        report = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                                 "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                                              {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant"}]})
        frozen = {"one": mutant}
        self.assertTrue(MODULE.mutation_counts(SCOPE, report, frozen, filename)["gate_passed"])
        for change in ("counter", "missing", "duplicate", "baseline", "test_status", "no_test", "test_args", "duration", "timeout", "bool_total"):
            bad = json.loads(json.dumps(report))
            if change == "counter": bad["caught"] = 0
            if change == "missing": bad["outcomes"].pop()
            if change == "duplicate": bad["outcomes"].append(bad["outcomes"][1])
            if change == "baseline": bad["outcomes"][0]["phase_results"][1]["process_status"] = {"Failure": 101}
            if change == "test_status": bad["outcomes"][1]["phase_results"][1]["process_status"] = "Success"
            if change == "no_test": bad["outcomes"][1]["phase_results"].pop()
            if change == "test_args": bad["outcomes"][1]["phase_results"][1]["argv"].append("--ignored")
            if change == "duration": bad["outcomes"][1]["phase_results"][1]["duration"] = float("nan")
            if change == "timeout": bad["outcomes"][1]["summary"] = "Timeout"
            if change == "bool_total": bad["total_mutants"] = True
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, bad, frozen, filename)
        for native in (2, 3, 4, 70, -9, None, True):
            with self.subTest(native=native), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(SCOPE, report, frozen, filename, native_exit_code=native)
        missed = complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                                  "outcomes": [{"scenario": "Baseline", "summary": "Success"},
                                               {"scenario": {"Mutant": mutant}, "summary": "MissedMutant"}]})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(SCOPE, missed, frozen, filename, native_exit_code=0, defer_gate=True)

    def test_only_the_exact_campaign_may_retain_native_exit_two(self):
        tools = Path("/admitted/tools"); filename = FILES[1][1]
        command = ["/admitted/tools/cargo-mutants", "mutants", "--package", "time-domain", "--file", filename,
                   "--test-workspace", "false", "--cap-lints", "false", "--jobs", "12",
                   "--jobserver", "true", "--jobserver-tasks", "12", "--baseline", "run",
                   "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/era"]
        result = {"exit_code": 2, "failure": None, "output": "two independently visible survivors"}
        self.assertIs(MODULE.mutation_campaign_result(SCOPE, tools, "era", command, result), result)
        for changed in (["/other/tool", *command[1:]], [*command, "--skip-calls-defaults"],
                        [word.replace(filename, FILES[0][1]) for word in command],
                        [word.replace("/work/mutation/era", "/work/other") for word in command],
                        [word if index != command.index("--jobs") + 1 else "13"
                         for index, word in enumerate(command)],
                        [word if index != command.index("--jobserver-tasks") + 1 else "13"
                         for index, word in enumerate(command)],
                        [word if index != command.index("--jobserver") + 1 else "false"
                         for index, word in enumerate(command)]):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", changed, result)
        for changed in ({**result, "exit_code": code} for code in (1, 3, 4, 70, -9, None, True)):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_result(SCOPE, tools, "era", command, changed)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_campaign_result(SCOPE, tools, "era", command, {**result, "failure": "deadline"})
        session = MODULE.Session.__new__(MODULE.Session)
        session.scope, session.tools = SCOPE, tools
        session.policy = {}
        with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run") as runtime:
            with self.assertRaises(MODULE.Incomplete):
                session.run("mutation-campaign:era", ["/other/tool"], 30, mutation_subject="era")
            runtime.assert_not_called()

    def test_artifact_must_be_the_real_library_of_the_exact_manifest(self):
        manifest = "/work/source/crates/time-domain/Cargo.toml"
        artifact = {"reason": "compiler-artifact", "package_id": "path+file:///work/source/crates/time-domain#0.0.0",
                    "manifest_path": manifest, "target": {"name": "time_domain", "kind": ["lib"]},
                    "filenames": ["/work/target/debug/libtime_domain.rlib"]}
        self.assertEqual(MODULE.library_artifact(json.dumps(artifact), "time-domain", manifest),
                         "/work/target/debug/libtime_domain.rlib")
        for patch in [{"manifest_path": "/unrelated/Cargo.toml"},
                      {"package_id": "registry+https://example#time-domain@0.0.0"},
                      {"filenames": ["/work/target/../stolen.rlib"]},
                      {"target": {"name": "time_domain", "kind": ["test"]}}]:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.library_artifact(json.dumps({**artifact, **patch}), "time-domain", manifest)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.library_artifact(json.dumps(artifact) + "\n" + json.dumps(artifact), "time-domain", manifest)

    def test_compiler_error_requires_exact_code_primary_span_and_type(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        diagnostic = self._external_diagnostic()
        result = {"exit_code": 1, "failure": None, "output": json.dumps(diagnostic)}
        MODULE.compiler_cause(result, specimen, path)
        for change in [{"code": {"code": "E0432", "explanation": None}}, {"message": "unrelated expected u8, found u16"},
                       {"spans": self._external_diagnostic(line=2)["spans"]}]:
            bad = {**result, "output": json.dumps({**diagnostic, **change})}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.compiler_cause(bad, specimen, path)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.compiler_cause({**result, "exit_code": 0}, specimen, path)

    def test_cause_tokens_can_come_from_real_rustc_span_labels(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 4}
        diagnostic = self._external_diagnostic(message="mismatched types", line=4)
        diagnostic["spans"][0]["label"] = "expected `MonotonicNs`, found `ActiveAwakeNs`"
        MODULE.compiler_cause({"exit_code": 1, "failure": None, "output": json.dumps(diagnostic)}, specimen, path)

    @staticmethod
    def _external_diagnostic(*, message="expected MonotonicNs, found ActiveAwakeNs",
                             code="E0308", level="error", line=3):
        """Literal rustc JSON envelope; no parser-generated expected values."""
        spans = [] if line is None else [{
            "file_name": "/work/contract-inputs/mixed.rs", "byte_start": 20, "byte_end": 21,
            "line_start": line, "line_end": line, "column_start": 1, "column_end": 2,
            "is_primary": True, "text": [{"text": "value", "highlight_start": 1, "highlight_end": 2}],
            "label": None, "suggested_replacement": None, "suggestion_applicability": None,
            "expansion": None,
        }]
        return {"$message_type": "diagnostic", "level": level,
                "code": None if code is None else {"code": code, "explanation": None},
                "message": message, "children": [], "spans": spans, "rendered": None}

    def test_compiler_rejects_extra_same_code_error_after_healthy_intended_cause(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        intended = self._external_diagnostic()
        healthy = {"exit_code": 1, "failure": None, "output": json.dumps(intended)}
        MODULE.compiler_cause(healthy, specimen, path)
        for unrelated in (self._external_diagnostic(message="expected u8, found u16"),
                          self._external_diagnostic(line=4)):
            for records in ([intended, unrelated], [unrelated, intended]):
                with self.subTest(records=records):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.compiler_cause({**healthy, "output": "\n".join(map(json.dumps, records))}, specimen, path)

    def test_compiler_rejects_unrelated_uncoded_and_nonjson_contamination(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        intended = json.dumps(self._external_diagnostic())
        contaminations = (
            json.dumps(self._external_diagnostic(code=None, message="failed to spawn compiler", line=None)),
            "error: could not execute process", "{", "[]", "null", "\n", "true",
            json.dumps({**self._external_diagnostic(), "spans": "invalid"}),
            json.dumps({**self._external_diagnostic(), "children": None}),
        )
        for contamination in contaminations:
            with self.subTest(contamination=contamination):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.compiler_cause({"exit_code": 1, "failure": None,
                                           "output": intended + "\n" + contamination}, specimen, path)

    def test_compiler_accepts_counted_native_summaries_and_multiple_matching_causes(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        intended = self._external_diagnostic()
        note = self._external_diagnostic(code=None, level="failure-note", line=None,
            message="For more information about this error, try `rustc --explain E0308`.")
        for errors, warnings in ((1, 0), (2, 0), (1, 1), (2, 2)):
            message = f"aborting due to {errors} previous error" + ("s" if errors > 1 else "")
            if warnings:
                message += f"; {warnings} warning" + ("s" if warnings > 1 else "") + " emitted"
            summary = self._external_diagnostic(code=None, line=None, message=message)
            warning = self._external_diagnostic(code="unused_imports", level="warning", message="unused import", line=1)
            warning["children"] = [{key: value for key, value in self._external_diagnostic(
                code=None, level="note", message="on by default", line=None).items() if key != "$message_type"}]
            records = [intended] * errors + [warning] * warnings + [summary, note]
            with self.subTest(errors=errors, warnings=warnings):
                MODULE.compiler_cause({"exit_code": 1, "failure": None,
                                       "output": "\n".join(map(json.dumps, records)) + "\n"}, specimen, path)

    def test_compiler_rejects_unbound_and_inconsistent_abort_or_explain_summaries(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        intended = self._external_diagnostic()
        summary = self._external_diagnostic(code=None, line=None, message="aborting due to 1 previous error")
        note = self._external_diagnostic(code=None, level="failure-note", line=None,
            message="For more information about this error, try `rustc --explain E0308`.")
        bad_rows = (
            [intended, {**summary, "message": "aborting due to 2 previous errors"}],
            [intended, {**summary, "message": "aborting due to 1 previous errors"}],
            [intended, {**summary, "message": "aborting due to 1 previous error; 1 warning emitted"}],
            [intended, summary, summary], [summary, intended], [intended, note],
            [intended, summary, {**note, "message": "For more information about this error, try `rustc --explain E0277`."}],
            [intended, summary, note, note], [intended, {**summary, "spans": intended["spans"]}],
            [intended, summary, {**note, "code": {"code": "E0308", "explanation": None}}],
            [intended, summary, intended],
        )
        for records in bad_rows:
            with self.subTest(records=records):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.compiler_cause({"exit_code": 1, "failure": None,
                                           "output": "\n".join(map(json.dumps, records))}, specimen, path)

    def test_compiler_rejects_illtyped_nested_duplicate_and_tool_diagnostics(self):
        path = "/work/contract-inputs/mixed.rs"
        specimen = {"code": "E0308", "tokens": ["MonotonicNs", "ActiveAwakeNs"], "line": 3}
        intended = self._external_diagnostic()
        nested = {key: value for key, value in intended.items() if key != "$message_type"}
        bad_rows = [
            {**intended, "children": [nested]}, {**intended, "$message_type": "artifact"},
            {**intended, "rendered": 1}, {**intended, "message": True},
            {**intended, "code": {"code": "E0308", "explanation": []}},
            {**intended, "spans": [{**intended["spans"][0], "is_primary": 1}]},
            {**intended, "spans": [{**intended["spans"][0], "line_start": True}]},
            {**intended, "spans": [{**intended["spans"][0], "label": []}]},
            {**intended, "spans": [{**intended["spans"][0], "text": "invalid"}]},
            {**intended, "message": intended["message"] + "; internal compiler error"},
            self._external_diagnostic(level="warning", code="unexpected_tool", message="tool failure"),
            self._external_diagnostic(level="warning", code="unused_imports", message="failed to spawn compiler"),
        ]
        raw_rows = [json.dumps(item) for item in bad_rows]
        raw_rows.extend((json.dumps(intended).replace('"level": "error"', '"level": "warning", "level": "error"'),
                         json.dumps(intended).replace('"rendered": null', '"rendered": NaN')))
        for raw in raw_rows:
            with self.subTest(raw=raw):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.compiler_cause({"exit_code": 1, "failure": None, "output": raw}, specimen, path)
        for malformed in ({"exit_code": True, "failure": None, "output": json.dumps(intended)},
                          {"exit_code": 1, "failure": "timeout", "output": json.dumps(intended)},
                          {"exit_code": 1, "failure": None, "output": b"invalid"},
                          {"exit_code": 1, "output": json.dumps(intended)}, None, []):
            with self.subTest(malformed=malformed):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.compiler_cause(malformed, specimen, path)

    def test_session_rechecks_new_copied_inputs_config_and_dependency_changes(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp); root = base / "checkout"; root.mkdir()
            work = base / "work"; work.mkdir(); source = work / "source"; source.mkdir()
            (root / "input").write_text("frozen"); (source / "input").write_text("frozen")
            config = work / "config.toml"; config.write_text("fixed")
            dependency = work / "deps"; dependency.mkdir(); (dependency / "lib.rlib").write_text("compiled")
            session = MODULE.Session.__new__(MODULE.Session)
            session.root, session.source = root, source
            session.identity = {"input": MODULE.PRODUCT.sha256(root / "input")}
            session.product_identity = dict(session.identity)
            session.source_inventory = MODULE.PRODUCT.inventory(source)
            session.config, session.config_sha256 = config, MODULE.PRODUCT.sha256(config)
            session.protected_tools = {}; session.protected_islands = {}
            session.protect(dependency)
            with patch.object(MODULE, "require_current_candidate"):
                session.check()
                added = source / "unexpected"; added.write_text("new")
                with self.assertRaises(MODULE.Incomplete): session.check()
                added.unlink(); config.write_text("changed")
                with self.assertRaises(MODULE.Incomplete): session.check()
                config.write_text("fixed"); (dependency / "lib.rlib").write_text("changed")
                with self.assertRaises(MODULE.Incomplete): session.check()

    def test_shared_lane_orchestration_retains_five_separate_denominators(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); tools = work / "tools"; tools.mkdir()
            report = {}; calls = []
            policy = {"coverage_wall_seconds": 300, "mutation_wall_seconds": 1200, "maximum_mutants": 512}
            warning = (" WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
                       "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
                       "are usually safe\n")
            def run(label, command, timeout, **options):
                calls.append((label, command, timeout))
                if label == "coverage":
                    (work / "coverage.json").write_text(json.dumps({"data": [{"files": [
                        {"filename": "/work/source/" + filename, "summary": {"lines": {"count": 10, "covered": 9}}}
                        for _, filename in FILES]}]}))
                    return "test result: ok. 8 passed; 0 failed; 0 ignored;"
                name = label.split(":")[1]; filename = dict(FILES)[name]
                mutant = {"file": filename, "name": "one", "package": "time-domain"}
                if label.startswith("mutation-enumeration:"):
                    return (warning if name == "era" else "") + json.dumps([mutant])
                directory = work / "mutation" / name / "mutants.out"; directory.mkdir(parents=True)
                (directory / "baseline.log").write_text("test result: ok. 8 passed; 0 failed; 0 ignored;")
                (directory / "outcomes.json").write_text(json.dumps(complete_report({"cargo_mutants_version": "27.1.0", "total_mutants": 1,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "log_path": "baseline.log"},
                                 {"scenario": {"Mutant": mutant}, "summary": "CaughtMutant", "phase_results": [{"phase": "Test"}]}]})))
                return {"exit_code": 0, "failure": None, "output": "completed"}
            MODULE.run_analysis_lanes(SCOPE, dict(FILES), policy, work, tools, run, report, "all")
            self.assertEqual(report["mutation_enumerated_total"], 5)
            self.assertEqual(report["mutation_enumeration"]["era"]["warning"], warning)
            self.assertEqual(report["mutation_enumeration"]["era"]["raw_output"], warning + json.dumps(
                [{"file": dict(FILES)["era"], "name": "one", "package": "time-domain"}]))
            self.assertIsNone(report["mutation_enumeration"]["coordinate"]["warning"])
            self.assertEqual(set(report["mutation"]), set(dict(FILES)))
            self.assertTrue(all(value["viable"] == 1 for value in report["mutation"].values()))
            self.assertEqual(len(calls), 11)
            self.assertTrue(all(command[command.index("--package") + 1] == "time-domain" for _, command, _ in calls))
            for label, command, _ in calls:
                if label.startswith("mutation-"):
                    self.assertEqual(command[command.index("--jobserver") + 1], "true")
                    self.assertEqual(command[command.index("--jobserver-tasks") + 1], "12")

    def test_session_seals_all_copied_dependencies_without_overlapping_binds(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            session = MODULE.Session.__new__(MODULE.Session)
            session.root, session.work, session.source = base / "root", base / "work", base / "work/source"
            session.output, session.config = base / "output", base / "work/cargo-home/config.toml"
            session.output.mkdir()
            session.base, session.stable, session.vendor, session.tools = (base / n for n in ("payload", "stable", "vendor", "tools"))
            session.admission_profile = "full"
            session.payload_readonly = (session.base, session.stable)
            session.admitted = {"policy": {"common_nightly": "nightly"}}
            session.protected_islands = {base / "work/deps": {}, base / "work/consumers": {}}
            session.protected_tools = {session.tools: {}}
            session.report = {"lanes": []}
            session.policy = {}
            captured = {}
            raw_listing = " WARN exact captured output\n[]\n"
            def runner(*args, **options):
                captured.update(options)
                output = raw_listing if "--list" in args[2] else "observed"
                return {"exit_code": 0, "failure": None, "output": output}
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                result = session.run("control", ["/tool", "arg"], 30,
                                     sealed=[base / "work/deps/lib.rlib", base / "work/consumers"])
            self.assertEqual(result, "observed")
            self.assertEqual(set(captured["sealed"]), {session.source, session.config, base / "work/deps", base / "work/consumers"})
            self.assertEqual((captured["timeout"], captured["cpu_seconds"]), (30, 30))
            self.assertEqual(captured["build_jobs"], 12)
            self.assertEqual(captured["environment"]["RUST_TEST_THREADS"], "12")
            session.policy = {"jobs": 6}
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                result = session.run("coverage", ["/tool", "arg"], 30)
            self.assertEqual(result, "observed")
            self.assertEqual(captured["build_jobs"], 6)
            self.assertEqual(captured["environment"]["RUST_TEST_THREADS"], "6")
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", side_effect=runner):
                with patch.object(MODULE.PRODUCT.ENV, "require_ok", return_value="trimmed") as require_ok:
                    result = session.run("mutation-enumeration:era", ["/tool", "--list"], 30)
            require_ok.assert_called_once()
            self.assertEqual(result, raw_listing)
            self.assertEqual(session.report["lanes"][-1]["output"], raw_listing)
            self.assertEqual(captured["build_jobs"], 1)
            self.assertEqual(captured["environment"]["RUST_TEST_THREADS"], "1")


    def test_history_success_preserves_raw_output_only_after_ordinary_success(self):
        raw = '  {"reason":"build-finished","success":true}\n\n'
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            session = MODULE.Session.__new__(MODULE.Session)
            session.root = session.output = base
            session.work, session.source, session.config = base / "work", base / "source", base / "config"
            session.base, session.stable, session.vendor, session.tools = (base / name for name in ("payload", "stable", "vendor", "tools"))
            session.admission_profile = "stable-core"
            session.payload_readonly = (session.base, session.stable)
            session.protected_islands, session.protected_tools = {}, {}
            session.admitted = {"policy": {"common_nightly": "night"}}
            session.policy, session.report = {"jobs": 6}, {"lanes": []}
            success = {"exit_code": 0, "failure": None, "output": raw, "command": ["synthetic-child"]}
            with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", return_value=success):
                for label in ("ke-history-contract-gnu-legal", "ke-history-contract-msvc-legal"):
                    self.assertEqual(session.run(label, ["cargo", "check"], 1), raw)
                    self.assertEqual(session.report["lanes"][-1]["output"], raw)
                self.assertEqual(session.run("ke-provider-contract-gnu-once", ["cargo", "check"], 1), raw.strip())
            for exit_code, failure in ((1, None), (0, "synthetic timeout")):
                result = {**success, "exit_code": exit_code, "failure": failure}
                with patch.object(session, "check"), patch.object(MODULE.RUNTIME, "run", return_value=result):
                    with self.subTest(exit_code=exit_code, failure=failure), self.assertRaises(MODULE.PRODUCT.ENV.Incomplete):
                        session.run("ke-history-contract-gnu-legal", ["cargo", "check"], 1)
                self.assertEqual(session.report["lanes"][-1], {"lane": "ke-history-contract-gnu-legal", **result})


class ReferenceInputAdmissionTests(unittest.TestCase):
    """The new singleton is an explicit listing/campaign role, never an operator."""

    SUBJECTS = (('reference_input', 'crates/time-core/src/reference_input.rs'),)
    CARD = 'tasks/s1.4a-reference-input-inventory-admission.json'

    def inventory(self):
        return MODULE.Scope('time-core', self.SUBJECTS, self.CARD), {
            'schema_version': 14, 'admission_status': 'admitted-reference-input-single-owner-inventory-only',
            'package': 'time-core', 'source_file_count': 154, 'subjects': dict(self.SUBJECTS),
            'tool_admission_profile': 'stable-core', 'maximum_mutants': 512,
            'listing_maximum_per_file': 512, 'listing_maximum_total': 512,
            'inventory_wall_seconds': 600, 'listing_process_limit': 128,
            'mutation_process_limit': 512, 'mutation_jobs': 10, 'mutation_message_format': 'json',
            'mutation_error_values': {}, 'mutation_equivalences': [],
            'native_test_invocations': 0, 'campaigns_admitted': False,
        }

    def campaign(self):
        return MODULE.Scope('time-core', self.SUBJECTS, 'policy/time-core-analysis.json'), {
            'schema_version': 23, 'admission_status': 'admitted-final-source', 'package': 'time-core',
            'source_file_count': 154, 'critical_files': [self.SUBJECTS[0][1]],
            'tool_admission_profile': 'stable-core', 'mutation_error_values': {},
            'mutation_equivalences': [],
        }

    def test_exact_singleton_listing_and_campaign_roles_admit_without_error_operator(self):
        scope, policy = self.inventory()
        self.assertEqual(MODULE.admission_profile(
            SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        self.assertEqual(MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], policy), [])
        scope, policy = self.campaign()
        for lane in ('contracts', 'coverage', 'mutation'):
            with self.subTest(lane=lane):
                self.assertEqual(MODULE.admission_profile(
                    SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        self.assertEqual(MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], policy), [])

    def test_singleton_inventory_requires_every_exact_typed_fact(self):
        scope, policy = self.inventory()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for key, value in (
            ('schema_version', 13), ('schema_version', True), ('source_file_count', 153),
            ('source_file_count', 154.0), ('package', 'time-domain'), ('subjects', {}),
            ('tool_admission_profile', 'full'), ('maximum_mutants', 513),
            ('listing_maximum_per_file', 513), ('listing_maximum_total', 4608),
            ('inventory_wall_seconds', 601), ('listing_process_limit', 512),
            ('mutation_process_limit', 128), ('mutation_jobs', True), ('mutation_jobs', 12),
            ('mutation_message_format', 'human'), ('mutation_equivalences', [{}]),
            ('mutation_error_values', None), ('mutation_error_values', []),
            ('mutation_error_values', {'reference_input': ['false']}),
            ('native_test_invocations', False), ('native_test_invocations', 1),
            ('campaigns_admitted', 0), ('campaigns_admitted', True),
            ('admission_status', 'admitted-final-source'),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_singleton_listing_scope_lane_and_profile_cannot_be_rebound(self):
        scope, policy = self.inventory()
        for wrong in (
            MODULE.Scope('time-domain', scope.subjects, scope.policy),
            MODULE.Scope(scope.package, (), scope.policy),
            MODULE.Scope(scope.package, (*scope.subjects, ('other', 'src/other.rs')), scope.policy),
            MODULE.Scope(scope.package, (('reference_input', 'src/other.rs'),), scope.policy),
            MODULE.Scope(scope.package, scope.subjects, 'policy/time-core-analysis.json'),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/s1.3-chart-combination-inventory-admission.json'),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json'),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'),
                                         wrong, policy)
        for lane in ('contracts', 'coverage', 'all', 'fuzz', 'inventory', 'cohort-union'):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='full'),
                                     scope, {**policy, 'tool_admission_profile': 'full'})

    def test_singleton_campaign_requires_exact_role_and_empty_proof_error_maps(self):
        scope, policy = self.campaign()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for key, value in (
            ('schema_version', 22), ('schema_version', True), ('source_file_count', 152),
            ('source_file_count', 154.0), ('package', 'time-domain'), ('critical_files', []),
            ('admission_status', 'foreign-role'), ('mutation_equivalences', None),
            ('mutation_equivalences', [{}]), ('mutation_error_values', None),
            ('mutation_error_values', []), ('mutation_error_values', {'reference_input': []}),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})
        for lane in ('all', 'fuzz', 'inventory', 'cohort-union'):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)
        for wrong in (
            MODULE.Scope('time-domain', scope.subjects, scope.policy),
            MODULE.Scope(scope.package, (), scope.policy),
            MODULE.Scope(scope.package, (*scope.subjects, ('other', 'src/other.rs')), scope.policy),
            MODULE.Scope(scope.package, (('reference_input', 'src/other.rs'),), scope.policy),
            MODULE.Scope(scope.package, MODULE.CHART_COMBINATION_SUBJECTS, scope.policy),
            MODULE.Scope(scope.package, scope.subjects, self.CARD),
            MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json'),
        ):
            with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, wrong, policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='full'),
                                     scope, {**policy, 'tool_admission_profile': 'full'})

    def test_singleton_error_maps_and_empty_equivalences_are_mandatory_in_both_roles(self):
        for scope, policy in (self.inventory(), self.campaign()):
            for key in ('mutation_error_values', 'mutation_equivalences'):
                for value in (None, False, [], {}, [{}], {'reference_input': []}, {'reference_input': ['false']}):
                    if value == ([] if key == 'mutation_equivalences' else {}) and type(value) is (list if key == 'mutation_equivalences' else dict):
                        continue
                    with self.subTest(role=scope.policy, key=key, value=value), self.assertRaises(MODULE.Incomplete):
                        MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], {**policy, key: value})
                with self.subTest(role=scope.policy, missing=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], {k: v for k, v in policy.items() if k != key})
            for filename in ('src/other.rs', 'crates/time-core/src/principal.rs', None):
                with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, policy)
            for wrong in (
                MODULE.Scope('time-domain', scope.subjects, scope.policy),
                MODULE.Scope(scope.package, (('reference_input', 'src/other.rs'),), scope.policy),
                MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json'),
            ):
                with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(wrong, self.SUBJECTS[0][1], policy)

    def test_singleton_listing_keeps_exact_commands_and_cannot_start_campaign(self):
        scope, policy = self.inventory()
        calls = []
        def run(label, command, timeout):
            self.assertEqual(command[-2:], ['--list', '--json'])
            self.assertNotIn('--error', command)
            self.assertNotIn('--baseline', command)
            calls.append(label)
            return json.dumps([{'name': label, 'file': self.SUBJECTS[0][1], 'package': 'time-core'}])
        found = MODULE.enumerate_mutation_inventories(
            scope, dict(scope.subjects), policy, Path('/sealed/tools'), run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=512)
        self.assertEqual(len(calls), 1)
        self.assertEqual(set(found.inventories), {'reference_input'})
        command = [*found.commands['reference_input'], '--baseline', 'run', '--build-timeout', '90',
                   '--timeout', '60', '--output', '/work/mutation/reference_input']
        with self.assertRaisesRegex(MODULE.Incomplete, 'listing only'):
            MODULE.mutation_campaign_command(scope, Path('/sealed/tools'), 'reference_input', command,
                expected_jobs=10, expected_message_format='json', policy=policy)
        current, current_policy = self.campaign()
        MODULE.mutation_campaign_command(current, Path('/sealed/tools'), 'reference_input', command,
            expected_jobs=10, expected_message_format='json', policy=current_policy)
        for extra in ('--error', '--skip-call-defaults', '--timeout=1'):
            with self.subTest(extra=extra), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(current, Path('/sealed/tools'), 'reference_input', [*command, extra],
                    expected_jobs=10, expected_message_format='json', policy=current_policy)


class AchievedPhaseAdmissionTests(ReferenceInputAdmissionTests):
    """S1.4c adds an exact role; its prior singleton remains separately bound."""

    CARD = 'tasks/s1.4c-achieved-phase-inventory-admission.json'

    def inventory(self):
        scope, policy = super().inventory()
        return scope, {**policy, 'schema_version': 16, 'source_file_count': 156,
                       'admission_status': 'admitted-achieved-phase-single-owner-inventory-only'}

    def campaign(self):
        scope, policy = super().campaign()
        return scope, {**policy, 'schema_version': 25, 'source_file_count': 156,
                       'mutation_jobs': 10, 'mutation_process_limit': 512,
                       'mutation_message_format': 'json'}

    def test_current_inventory_error_map_reaches_shared_helper(self):
        scope, policy = self.inventory()
        self.assertEqual(MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], policy), [])

    def test_current_campaign_matches_exact_final_source_role(self):
        scope, policy = self.campaign()
        for lane in ('contracts', 'coverage', 'mutation'):
            with self.subTest(lane=lane):
                self.assertEqual(MODULE.admission_profile(
                    SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')

    def test_singleton_generations_remain_typed_and_bound_at_both_entrypoints(self):
        legacy = ReferenceInputAdmissionTests()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for scope, policy in (self.inventory(), self.campaign(), legacy.inventory(), legacy.campaign()):
            changes = (
                ('schema_version', policy['schema_version'] + 1), ('schema_version', True),
                ('schema_version', float(policy['schema_version'])),
                ('source_file_count', policy['source_file_count'] + 1), ('source_file_count', True),
                ('source_file_count', float(policy['source_file_count'])),
                ('admission_status', 'foreign-role'), ('package', 'time-domain'),
            )
            for key, value in changes:
                for entrypoint in ('profile', 'errors'):
                    with self.subTest(role=scope.policy, schema=policy['schema_version'],
                                      key=key, value=value, entrypoint=entrypoint), self.assertRaises(MODULE.Incomplete):
                        changed = {**policy, key: value}
                        if entrypoint == 'profile':
                            MODULE.admission_profile(args, scope, changed)
                        else:
                            MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], changed)
            for key in ('schema_version', 'source_file_count', 'admission_status', 'package'):
                with self.subTest(role=scope.policy, missing=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(
                        scope, self.SUBJECTS[0][1], {name: value for name, value in policy.items() if name != key})

    def test_current_card_cannot_be_renamed_or_borrow_legacy_role(self):
        scope, policy = self.inventory()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        legacy = ReferenceInputAdmissionTests()
        old_scope, old_policy = legacy.inventory()
        for wrong_scope, wrong_policy in (
            (old_scope, policy), (scope, old_policy),
            (MODULE.Scope(scope.package, scope.subjects, 'tasks/s1.4c-renamed.json'), policy),
            (scope, {**policy, 'admission_status': old_policy['admission_status']}),
            (old_scope, {**old_policy, 'admission_status': policy['admission_status']}),
        ):
            with self.subTest(scope=wrong_scope, status=wrong_policy['admission_status']):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, wrong_scope, wrong_policy)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(wrong_scope, self.SUBJECTS[0][1], wrong_policy)

    def test_current_campaign_cannot_mix_prior_schema_or_source_generation(self):
        scope, policy = self.campaign()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for changed in ({**policy, 'schema_version': 23}, {**policy, 'source_file_count': 154}):
            with self.subTest(schema=changed['schema_version'], source_count=changed['source_file_count']):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, scope, changed)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], changed)

    def test_current_campaign_keeps_exact_mutation_budgets_and_message_format(self):
        scope, policy = self.campaign()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for key, value in (
            ('mutation_jobs', True), ('mutation_jobs', 10.0), ('mutation_jobs', 12),
            ('mutation_process_limit', True), ('mutation_process_limit', 512.0),
            ('mutation_process_limit', 128), ('mutation_process_limit', 513),
            ('mutation_message_format', 'human'),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})


class ChartCompositionAdmissionTests(unittest.TestCase):
    """The new three-owner role cannot inherit historical discovery or operators."""

    SUBJECTS = (('chart', 'crates/time-core/src/regression/chart.rs'),
                ('chart_roundoff', 'crates/time-core/src/regression/chart/roundoff.rs'),
                ('chart_batch', 'crates/time-core/src/chart_batch.rs'))
    CARD = 'tasks/s1.4b-chart-composition-inventory-admission.json'

    def reviewed_equivalence(self):
        """Literal descriptor from the retained V2 native listing, without diff."""
        return {
            'id': 'S14B-R-E-001',
            'candidate_sha256': '6f5471fe21d35455a0be6b8f85800cb44d2b1e2f8effd8aec3492e840f0bf8c6',
            'source_sha256': '1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc',
            'mutant': {
                'name': 'crates/time-core/src/regression/chart/roundoff.rs:350:33: replace || with && in transform',
                'package': 'time-core', 'file': 'crates/time-core/src/regression/chart/roundoff.rs',
                'function': {'function_name': 'transform',
                             'return_type': '-> Result<ChartScalars, FixtureChartError>',
                             'span': {'start': {'line': 310, 'column': 1},
                                      'end': {'line': 376, 'column': 2}}},
                'span': {'start': {'line': 350, 'column': 33},
                         'end': {'line': 350, 'column': 35}},
                'replacement': '&&', 'genre': 'BinaryOperator'},
            'requirement': 'CHECK-7/8/14/25',
            'rationale': 'Complete checked chart construction gives both zero or both positive coefficient errors.',
            'proof': {'path': 'tasks/s1.4b-chart-composition-equivalence.md',
                      'sha256': '67a3b6eb23ef5f476d7f7af09ffb803863a470d2c9bafd3735e28f1397046231',
                      'section': 'S14B-R-E-001', 'reviewer': 'root; roundoff_survivor_review',
                      'verdict': 'independently-justified-equivalent'}}

    def test_one_reviewed_composition_equivalence_is_campaign_only(self):
        scope, policy = self.campaign()
        policy['mutation_equivalences'] = [self.reviewed_equivalence()]
        for lane in ('contracts', 'coverage', 'mutation'):
            with self.subTest(lane=lane):
                self.assertEqual(MODULE.admission_profile(
                    SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        for _, filename in self.SUBJECTS:
            self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])
        listing, card = self.inventory()
        card['mutation_equivalences'] = copy.deepcopy(policy['mutation_equivalences'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'), listing, card)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.per_subject_mutation_error_args(listing, self.SUBJECTS[0][1], card)

    def test_composition_equivalence_rejects_neighbors_duplicates_and_foreign_bindings(self):
        scope, policy = self.campaign()
        entry = self.reviewed_equivalence()
        bad = [None, {}, [entry, entry], [{}]]
        for key, value in (('id', 'S158-R-E-001'), ('candidate_sha256', '0' * 64),
                           ('source_sha256', '0' * 64), ('requirement', 'CHECK-7'),
                           ('rationale', ''), ('proof', {**entry['proof'], 'sha256': '0' * 64})):
            bad.append([{**entry, key: value}])
        for column in (26, 53):
            neighbor = copy.deepcopy(entry)
            neighbor['mutant'].update(
                name='crates/time-core/src/regression/chart/roundoff.rs:350:' + str(column) + ': replace != with == in transform',
                replacement='==', span={'start': {'line': 350, 'column': column},
                                        'end': {'line': 350, 'column': column + 2}})
            bad.append([neighbor])
        for entries in bad:
            changed = {**policy, 'mutation_equivalences': entries}
            with self.subTest(entries=entries), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'), scope, changed)
            with self.subTest(entries=entries), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], changed)

    def test_composition_e_keeps_synthetic_raw_miss_and_native_exit_separate(self):
        scope, _ = self.campaign()
        entry = self.reviewed_equivalence()
        equivalent = entry['mutant']
        neighbor = copy.deepcopy(equivalent)
        neighbor.update(name='synthetic-non-equivalent-neighbor', replacement='==')
        filename = equivalent['file']
        expected = {row['name']: row for row in (equivalent, neighbor)}
        outcomes = [{'scenario': 'Baseline', 'summary': 'Success'},
                    {'scenario': {'Mutant': neighbor}, 'summary': 'CaughtMutant'},
                    {'scenario': {'Mutant': equivalent}, 'summary': 'MissedMutant'}]
        def report_for(records):
            value = complete_report({'cargo_mutants_version': '27.1.0', 'total_mutants': 2, 'outcomes': records})
            for outcome in value['outcomes']:
                for phase in outcome['phase_results']:
                    phase['argv'] = [part.replace('--package=time-domain@0.0.0', '--package=time-core@0.0.0')
                                     for part in phase['argv']]
                    phase['argv'].insert(-1, '--message-format=json')
            return value
        report = report_for(outcomes)
        score = MODULE.mutation_counts(scope, report, expected, filename, native_exit_code=2, equivalences=[entry])
        self.assertEqual((score['killed'], score['survived'], score['unviable'], score['excluded']), (1, 1, 0, 1))
        self.assertEqual((score['raw_denominator'], score['adjusted_denominator'], score['native_exit_code']), (2, 1, 2))
        self.assertEqual((score['raw_viable_killed_percent'], score['adjusted_viable_killed_percent']), (50, 100))
        self.assertTrue(score['gate_passed'])
        for summary in ('CaughtMutant', 'Unviable', 'Timeout'):
            changed = copy.deepcopy(outcomes)
            changed[-1]['summary'] = summary
            bad = report_for(changed)
            with self.subTest(summary=summary), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_counts(scope, bad, expected, filename, native_exit_code=0, equivalences=[entry])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(scope, report, expected, filename, native_exit_code=0, equivalences=[entry])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.mutation_counts(scope, report, expected, filename, native_exit_code=2,
                                   equivalences=[{**entry, 'mutant': neighbor}])

    def inventory(self):
        return MODULE.Scope('time-core', self.SUBJECTS, self.CARD), {
            'schema_version': 15, 'admission_status': 'admitted-chart-composition-three-owner-inventory-only',
            'package': 'time-core', 'source_file_count': 156, 'subjects': dict(self.SUBJECTS),
            'tool_admission_profile': 'stable-core', 'maximum_mutants': 512,
            'listing_maximum_per_file': 512, 'listing_maximum_total': 1536,
            'inventory_wall_seconds': 600, 'listing_process_limit': 128,
            'mutation_process_limit': 512, 'mutation_jobs': 10, 'mutation_message_format': 'json',
            'mutation_error_values': {}, 'mutation_equivalences': [],
            'native_test_invocations': 0, 'campaigns_admitted': False,
        }

    def campaign(self):
        return MODULE.Scope('time-core', self.SUBJECTS, 'policy/time-core-analysis.json'), {
            'schema_version': 24, 'admission_status': 'admitted-final-source', 'package': 'time-core',
            'source_file_count': 156, 'critical_files': list(dict(self.SUBJECTS).values()),
            'tool_admission_profile': 'stable-core', 'mutation_error_values': {},
            'mutation_equivalences': [], 'mutation_jobs': 10, 'mutation_process_limit': 512,
            'mutation_message_format': 'json',
        }

    def test_exact_composition_listing_and_campaign_roles_admit(self):
        scope, policy = self.inventory()
        self.assertEqual(MODULE.admission_profile(
            SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        scope, policy = self.campaign()
        for lane in ('contracts', 'coverage', 'mutation'):
            with self.subTest(lane=lane):
                self.assertEqual(MODULE.admission_profile(
                    SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')
        for scope, policy in (self.inventory(), self.campaign()):
            for _, filename in self.SUBJECTS:
                self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])

    def test_composition_roles_require_exact_typed_facts_and_empty_maps(self):
        for scope, policy in (self.inventory(), self.campaign()):
            args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
            changes = [('schema_version', True), ('schema_version', 23), ('source_file_count', 154),
                       ('source_file_count', 156.0), ('package', 'time-domain'),
                       ('mutation_equivalences', [{}]), ('mutation_equivalences', {}),
                       ('mutation_error_values', {'chart': []}), ('mutation_error_values', []),
                       ('tool_admission_profile', 'full'), ('mutation_jobs', 12),
                       ('mutation_process_limit', 128), ('mutation_message_format', 'human')]
            if scope.policy == self.CARD:
                changes += [('subjects', {}), ('maximum_mutants', 513), ('listing_maximum_per_file', 513),
                            ('listing_maximum_total', 512), ('listing_maximum_total', 1537),
                            ('listing_process_limit', 512), ('inventory_wall_seconds', 601),
                            ('native_test_invocations', False), ('native_test_invocations', 1),
                            ('campaigns_admitted', 0), ('campaigns_admitted', True)]
            else:
                changes += [('critical_files', list(dict(MODULE.CHART_PRINCIPAL_SUBJECTS).values()))]
            for key, value in changes:
                with self.subTest(role=scope.policy, key=key, value=value), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, scope, {**policy, key: value})
            for key in policy:
                with self.subTest(role=scope.policy, missing=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_composition_scope_lane_and_profile_are_closed(self):
        for scope, policy in (self.inventory(), self.campaign()):
            for wrong in (MODULE.Scope('time-domain', scope.subjects, scope.policy),
                          MODULE.Scope(scope.package, scope.subjects[:-1], scope.policy),
                          MODULE.Scope(scope.package, (*scope.subjects, scope.subjects[0]), scope.policy),
                          MODULE.Scope(scope.package, MODULE.CHART_PRINCIPAL_SUBJECTS, scope.policy),
                          MODULE.Scope(scope.package, MODULE.REFERENCE_INPUT_SUBJECTS, scope.policy),
                          MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
                          MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed.json')):
                with self.subTest(role=scope.policy, scope=wrong), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='stable-core'),
                                             wrong, policy)
            lanes = ('contracts', 'coverage', 'all', 'fuzz', 'cohort-union') if scope.policy == self.CARD else ('all', 'fuzz', 'cohort-union')
            for lane in lanes:
                with self.subTest(role=scope.policy, lane=lane), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='full'),
                                         scope, {**policy, 'tool_admission_profile': 'full'})

    def test_composition_error_map_cannot_supply_an_operator_or_omit_empty_fields(self):
        for scope, policy in (self.inventory(), self.campaign()):
            for key, values in (('mutation_error_values', (None, [], {'chart': []}, {'chart': ['false']})),
                                ('mutation_equivalences', (None, {}, [{}]))):
                for value in values:
                    for _, filename in self.SUBJECTS:
                        with self.subTest(role=scope.policy, key=key, value=value), self.assertRaises(MODULE.Incomplete):
                            MODULE.per_subject_mutation_error_args(scope, filename, {**policy, key: value})
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1],
                                                          {k: v for k, v in policy.items() if k != key})
            for filename in ('src/foreign.rs', MODULE.REFERENCE_INPUT_SUBJECTS[0][1], None):
                with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, policy)

    def test_composition_listing_is_three_commands_and_grants_no_campaign(self):
        scope, policy = self.inventory()
        calls = []
        def run(label, command, timeout):
            subject = label.removeprefix('mutation-enumeration:')
            self.assertEqual(command[-2:], ['--list', '--json'])
            self.assertNotIn('--error', command)
            self.assertNotIn('--baseline', command)
            calls.append(subject)
            return json.dumps([{'name': label, 'file': dict(self.SUBJECTS)[subject], 'package': 'time-core'}])
        found = MODULE.enumerate_mutation_inventories(
            scope, dict(scope.subjects), policy, Path('/sealed/tools'), run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=1536)
        self.assertEqual(calls, [name for name, _ in self.SUBJECTS])
        self.assertEqual(set(found.inventories), set(dict(self.SUBJECTS)))
        for subject, _ in self.SUBJECTS:
            command = [*found.commands[subject], '--baseline', 'run', '--build-timeout', '90',
                       '--timeout', '60', '--output', '/work/mutation/' + subject]
            with self.assertRaisesRegex(MODULE.Incomplete, 'listing only'):
                MODULE.mutation_campaign_command(scope, Path('/sealed/tools'), subject, command,
                    expected_jobs=10, expected_message_format='json', policy=policy)


class SettledRateAdmissionTests(unittest.TestCase):
    """The four-owner settled role adds no schema/path/equivalence aliases."""

    SUBJECTS = (
        ('reference_input', 'crates/time-core/src/reference_input.rs'),
        ('rate_delivery', 'crates/time-core/src/reference_input/rate_delivery.rs'),
        ('chart', 'crates/time-core/src/regression/chart.rs'),
        ('chart_roundoff', 'crates/time-core/src/regression/chart/roundoff.rs'),
    )
    CARD = 'tasks/settled-rate-inventory-admission.json'

    def inventory(self):
        """Literal next-role facts; source/tool byte admission remains separate."""
        return MODULE.Scope('time-core', self.SUBJECTS, self.CARD), {
            'schema_version': 17, 'admission_status': 'admitted-settled-rate-four-owner-inventory-only',
            'package': 'time-core', 'source_file_count': 158, 'subjects': dict(self.SUBJECTS),
            'tool_admission_profile': 'stable-core', 'maximum_mutants': 512,
            'listing_maximum_per_file': 512, 'listing_maximum_total': 2048,
            'inventory_wall_seconds': 600, 'listing_process_limit': 128,
            'mutation_process_limit': 512, 'mutation_jobs': 10, 'mutation_message_format': 'json',
            'mutation_error_values': {}, 'mutation_equivalences': [],
            'native_test_invocations': 0, 'campaigns_admitted': False,
        }

    def campaign(self):
        """Four exact source owners, with no predecessor proof or error operator."""
        return MODULE.Scope('time-core', self.SUBJECTS, 'policy/time-core-analysis.json'), {
            'schema_version': 26, 'admission_status': 'admitted-final-source',
            'package': 'time-core', 'source_file_count': 158,
            'critical_files': [filename for _, filename in self.SUBJECTS],
            'tool_admission_profile': 'stable-core', 'mutation_error_values': {},
            'mutation_equivalences': [], 'mutation_jobs': 10,
            'mutation_process_limit': 512, 'mutation_message_format': 'json', 'maximum_mutants': 512,
        }

    def test_settled_roles_admit_exact_four_owners_without_error_operator(self):
        for scope, policy in (self.inventory(), self.campaign()):
            lanes = ('mutation',) if scope.policy == self.CARD else ('contracts', 'coverage', 'mutation')
            for lane in lanes:
                with self.subTest(role=scope.policy, lane=lane):
                    self.assertEqual(MODULE.admission_profile(
                        SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy), 'stable-core')
            for _, filename in self.SUBJECTS:
                with self.subTest(role=scope.policy, filename=filename):
                    self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), [])

    def test_settled_inventory_requires_every_exact_typed_fact(self):
        scope, policy = self.inventory()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for key, value in (
            ('schema_version', 16), ('schema_version', 18), ('schema_version', True), ('schema_version', 17.0),
            ('source_file_count', 156), ('source_file_count', 159), ('source_file_count', True),
            ('source_file_count', 158.0), ('package', 'time-domain'), ('subjects', {}),
            ('admission_status', 'admitted-achieved-phase-single-owner-inventory-only'),
            ('maximum_mutants', 513), ('listing_maximum_per_file', 513),
            ('listing_maximum_total', 512), ('listing_maximum_total', 2048.0),
            ('inventory_wall_seconds', 601), ('listing_process_limit', 512),
            ('mutation_process_limit', 128), ('mutation_jobs', True), ('mutation_jobs', 12),
            ('mutation_message_format', 'human'), ('mutation_equivalences', [{}]),
            ('mutation_error_values', None), ('mutation_error_values', []),
            ('native_test_invocations', False), ('native_test_invocations', 1),
            ('campaigns_admitted', 0), ('campaigns_admitted', True),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_settled_campaign_requires_exact_generation_scope_and_worker_policy(self):
        scope, policy = self.campaign()
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        for key, value in (
            ('schema_version', 25), ('schema_version', 27), ('schema_version', True), ('schema_version', 26.0),
            ('source_file_count', 156), ('source_file_count', 159), ('source_file_count', True),
            ('source_file_count', 158.0), ('package', 'time-domain'), ('critical_files', []),
            ('critical_files', list(reversed(policy['critical_files']))),
            ('admission_status', 'foreign-role'), ('mutation_equivalences', [{}]),
            ('mutation_error_values', None), ('mutation_error_values', []),
            ('mutation_jobs', True), ('mutation_jobs', 10.0), ('mutation_jobs', 12),
            ('mutation_process_limit', True), ('mutation_process_limit', 512.0),
            ('mutation_process_limit', 128), ('mutation_message_format', 'human'),
            ('maximum_mutants', 511), ('maximum_mutants', True), ('maximum_mutants', 513),
        ):
            with self.subTest(key=key, value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {**policy, key: value})
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_settled_scope_path_lane_and_profile_cannot_be_rebound(self):
        for scope, policy in (self.inventory(), self.campaign()):
            args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
            for wrong in (
                MODULE.Scope('time-domain', scope.subjects, scope.policy),
                MODULE.Scope(scope.package, (), scope.policy),
                MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
                MODULE.Scope(scope.package, scope.subjects[:-1], scope.policy),
                MODULE.Scope(scope.package, (*scope.subjects, ('other', 'src/other.rs')), scope.policy),
                MODULE.Scope(scope.package, MODULE.CHART_COMBINATION_SUBJECTS, scope.policy),
                MODULE.Scope(scope.package, (('reference_input', 'src/other.rs'), *scope.subjects[1:]), scope.policy),
                MODULE.Scope(scope.package, scope.subjects, 'tasks/renamed-settled-rate.json'),
                MODULE.Scope(scope.package, scope.subjects, 'tasks/s1.4c-achieved-phase-inventory-admission.json'),
            ):
                with self.subTest(scope=wrong), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, wrong, policy)
                with self.subTest(error_scope=wrong), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(wrong, self.SUBJECTS[0][1], policy)
            bad_lanes = ('contracts', 'coverage', 'all', 'fuzz', 'inventory', 'cohort-union') if scope.policy == self.CARD else ('all', 'fuzz', 'inventory', 'cohort-union')
            for lane in bad_lanes:
                with self.subTest(role=scope.policy, lane=lane), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile='stable-core'), scope, policy)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane='mutation', tool_admission_profile='full'),
                                         scope, {**policy, 'tool_admission_profile': 'full'})

    def test_settled_error_arguments_require_exact_role_and_empty_operator_fields(self):
        for scope, policy in (self.inventory(), self.campaign()):
            for field, bad_values in (
                ('mutation_error_values', (None, [], False, {'rate_delivery': []}, {'chart_roundoff': ['false']})),
                ('mutation_equivalences', (None, {}, (), False, [{}])),
                ('schema_version', (True, float(policy['schema_version']), policy['schema_version'] + 1)),
                ('source_file_count', (True, 158.0, 156, 159)),
                ('package', ('time-domain',)), ('admission_status', ('foreign-role',)),
                ('tool_admission_profile', ('full',)),
                ('mutation_jobs', (True, 12)), ('mutation_process_limit', (True, 128)),
                ('maximum_mutants', (True, 511, 513)),
                ('mutation_message_format', ('human',)),
            ):
                for value in bad_values:
                    for _, filename in self.SUBJECTS:
                        with self.subTest(role=scope.policy, field=field, value=value, filename=filename), self.assertRaises(MODULE.Incomplete):
                            MODULE.per_subject_mutation_error_args(scope, filename, {**policy, field: value})
                with self.subTest(role=scope.policy, missing=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1],
                        {key: value for key, value in policy.items() if key != field})
            for filename in ('src/other.rs', 'crates/time-core/src/chart_batch.rs', None):
                with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, policy)
            for old_subjects in (MODULE.CHART_COMBINATION_SUBJECTS, CORE_FILES):
                legacy = MODULE.Scope(scope.package, old_subjects, scope.policy)
                with self.subTest(legacy_fallthrough=legacy), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(legacy, 'crates/time-core/src/regression/chart.rs', policy)
                with self.subTest(legacy_missing_map=legacy), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(legacy, 'crates/time-core/src/regression/chart.rs',
                        {key: value for key, value in policy.items() if key != 'mutation_error_values'})
            role_field = 'subjects' if scope.policy == self.CARD else 'critical_files'
            for changed in ({**policy, role_field: {}},
                            {key: value for key, value in policy.items() if key != role_field}):
                with self.subTest(role_field=role_field), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1], changed)

    def test_settled_listing_keeps_four_exact_commands_and_never_starts_campaign(self):
        scope, policy = self.inventory()
        tools = Path('/sealed/tools')
        calls = []
        def run(label, command, timeout):
            subject = label.removeprefix('mutation-enumeration:')
            filename = dict(self.SUBJECTS)[subject]
            self.assertEqual(command, [str(tools / 'cargo-mutants'), 'mutants', '--package', 'time-core',
                '--file', filename, '--test-workspace', 'false', '--cap-lints', 'false', '--jobs', '10',
                '--jobserver', 'true', '--jobserver-tasks', '10', '--cargo-arg=--message-format=json', '--list', '--json'])
            calls.append(subject)
            return json.dumps([{'name': label, 'file': filename, 'package': 'time-core'}])
        found = MODULE.enumerate_mutation_inventories(
            scope, dict(scope.subjects), policy, tools, run, {},
            deadline=MODULE.time.monotonic() + 60, maximum_per_file=512, maximum_total=2048)
        self.assertEqual(calls, [name for name, _ in self.SUBJECTS])
        self.assertEqual(set(found.inventories), set(dict(self.SUBJECTS)))
        current, campaign = self.campaign()
        for subject, _ in self.SUBJECTS:
            command = [*found.commands[subject], '--baseline', 'run', '--build-timeout', '90',
                       '--timeout', '60', '--output', '/work/mutation/' + subject]
            with self.assertRaisesRegex(MODULE.Incomplete, 'listing only'):
                MODULE.mutation_campaign_command(scope, tools, subject, command,
                    expected_jobs=10, expected_message_format='json', policy=policy)
            MODULE.mutation_campaign_command(current, tools, subject, command,
                expected_jobs=10, expected_message_format='json', policy=campaign)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(current, tools, subject, [*command, '--error', 'false'],
                    expected_jobs=10, expected_message_format='json', policy=campaign)

    def test_settled_role_cannot_transfer_the_historical_roundoff_equivalence(self):
        previous = ChartCompositionAdmissionTests()
        old_scope, old_policy = previous.campaign()
        proof = [previous.reviewed_equivalence()]
        old_policy['mutation_equivalences'] = proof
        args = SimpleNamespace(lane='mutation', tool_admission_profile='stable-core')
        self.assertEqual(MODULE.admission_profile(args, old_scope, old_policy), 'stable-core')
        for scope, policy in (self.inventory(), self.campaign()):
            changed = {**policy, 'mutation_equivalences': proof}
            with self.subTest(role=scope.policy), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, changed)
            for _, filename in self.SUBJECTS:
                with self.subTest(role=scope.policy, filename=filename), self.assertRaises(MODULE.Incomplete):
                    MODULE.per_subject_mutation_error_args(scope, filename, changed)


class DomainLeapAdmissionTests(unittest.TestCase):
    """The new domain role cannot borrow unrelated or nightly tool custody."""

    SUBJECTS = (
        ("coordinate", "crates/time-domain/src/coordinate.rs"),
        ("era", "crates/time-domain/src/era.rs"),
        ("exchange", "crates/time-domain/src/exchange.rs"),
        ("clock", "crates/time-domain/src/clock.rs"),
        ("rounding", "crates/time-domain/src/rounding.rs"),
        ("leap", "crates/time-domain/src/leap.rs"),
        ("leap_validation", "crates/time-domain/src/leap/validation.rs"),
        ("leap_interpretation", "crates/time-domain/src/leap/interpretation.rs"),
        ("leap_bundled", "crates/time-domain/src/leap/bundled.rs"),
    )

    def fixture(self):
        """Independent literal role facts, without reading current policy."""
        return MODULE.Scope("time-domain", self.SUBJECTS, "policy/time-domain-analysis.json"), {
            "schema_version": 4, "package": "time-domain", "source_file_count": 166,
            "critical_files": [filename for _, filename in self.SUBJECTS],
            "tool_admission_profile": "stable-core", "mutation_equivalences": [],
            "mutation_error_values": {
                "leap_bundled": [
                    "{ let _ = (CONTINUOUS_FROM, DATA_REVISION, FINITE_FUTURE_BEFORE, HISTORY_COMPLETE_BEFORE, "
                    "LEAP_RECORDS, PROVENANCE_SHA256, RAW_SHA256, LeapContinuousFrom::new, LeapDataIdentity::new, "
                    "LeapFutureExpiry::new, LeapHistoryBefore::new); LeapError::InvalidRecords }"],
            }, "maximum_mutants": 512,
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
            "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "contracts_wall_seconds": 300, "jobs": 6, "test_threads": 12,
            "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
        }

    def test_exact_domain_leap_role_uses_existing_stable_custody(self):
        scope, policy = self.fixture()
        for lane in ("coverage", "mutation"):
            with self.subTest(lane=lane):
                args = SimpleNamespace(lane=lane, tool_admission_profile="stable-core")
                self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
        with self.assertRaisesRegex(MODULE.Incomplete, "exact nine-owner"):
            MODULE.admission_profile(SimpleNamespace(lane="contracts", tool_admission_profile="stable-core"), scope, policy)

    def test_each_domain_role_field_is_required_with_exact_types(self):
        scope, policy = self.fixture()
        args = SimpleNamespace(lane="coverage", tool_admission_profile="stable-core")
        for key, value in policy.items():
            alternatives = [None, True] if type(value) is not bool else [None, 1]
            if type(value) is int:
                alternatives.extend([float(value), value - 1, value + 1])
            elif type(value) is list:
                alternatives.append(["foreign.rs"])
            elif type(value) is dict:
                alternatives.append({"foreign.rs": "Ok(())"})
            else:
                alternatives.append("foreign")
            for changed in alternatives:
                with self.subTest(key=key, changed=changed), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, scope, {**policy, key: changed})
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})

    def test_domain_role_rejects_altered_scope_and_unsupported_lanes(self):
        scope, policy = self.fixture()
        args = SimpleNamespace(lane="coverage", tool_admission_profile="stable-core")
        for changed in (
                MODULE.Scope("time-core", self.SUBJECTS, scope.policy),
                MODULE.Scope(scope.package, self.SUBJECTS, "policy/foreign.json"),
                MODULE.Scope(scope.package, tuple(reversed(self.SUBJECTS)), scope.policy),
                MODULE.Scope(scope.package, self.SUBJECTS[:-1], scope.policy),
                MODULE.Scope(scope.package, (*self.SUBJECTS, ("generated", "crates/time-domain/src/leap/generated.rs")), scope.policy),
                MODULE.Scope(scope.package, list(self.SUBJECTS), scope.policy)):
            with self.subTest(scope=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, changed, policy)
        for lane in ("fuzz", "all", "inventory", "cohort-union", "unknown", None):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)

    def test_new_domain_role_has_no_full_profile_fallback(self):
        scope, policy = self.fixture()
        for lane in ("contracts", "coverage", "mutation", "fuzz", "all"):
            for profile in (None, "full", "stable-domain-leap"):
                with self.subTest(lane=lane, profile=profile), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile=profile),
                                             scope, {**policy, "tool_admission_profile": profile})
        self.assertEqual(MODULE.admission_profile(SimpleNamespace(lane="fuzz"), SCOPE, {}), "full")

    def test_domain_session_checks_actual_product_count_before_tools_or_output(self):
        scope, policy = self.fixture()
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary).resolve()
            root = parent / "checkout"
            root.mkdir()
            args = SimpleNamespace(lane="coverage", tool_admission_profile="stable-core",
                                   output_dir=parent / "output")
            for count in (165, 166, 167):
                identity = {f"source/{index}.rs": "a" * 64 for index in range(count)}
                expected = ("healthy product reached proof admission" if count == 166
                            else "product input count")
                with (self.subTest(count=count),
                      patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)),
                      patch.object(MODULE, "admit_mutation_equivalences",
                                   side_effect=MODULE.Incomplete("healthy product reached proof admission")) as proofs,
                      patch.object(MODULE.ANALYSIS, "admit") as tools,
                      self.assertRaisesRegex(MODULE.Incomplete, expected)):
                    MODULE.Session(root, args, {}, scope, policy, ())
                self.assertEqual(proofs.call_count, int(count == 166))
                tools.assert_not_called()
                self.assertFalse(args.output_dir.exists())

    def test_domain_mutation_error_policy_accepts_only_exact_role_and_literal_map(self):
        scope, policy = self.fixture()
        for name, filename in self.SUBJECTS:
            with self.subTest(filename=filename):
                expected = (["--error", policy["mutation_error_values"]["leap_bundled"][0]]
                            if name == "leap_bundled" else [])
                self.assertEqual(MODULE.per_subject_mutation_error_args(scope, filename, policy), expected)
        for filename in (None, "src/foreign.rs", "crates/time-domain/src/leap/generated.rs"):
            with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, filename, policy)
        for key, value in policy.items():
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1],
                                                       {k: v for k, v in policy.items() if k != key})
        for mapping in (None, {}, [], {"crates/time-domain/src/leap.rs": "Ok(Default::default())"}):
            with self.subTest(mapping=mapping), self.assertRaises(MODULE.Incomplete):
                MODULE.per_subject_mutation_error_args(scope, self.SUBJECTS[0][1],
                                                       {**policy, "mutation_error_values": mapping})

    def test_domain_campaign_preserves_the_exact_bounded_native_command(self):
        scope, policy = self.fixture()
        tools = Path("/owned/tools")
        command = [str(tools / "cargo-mutants"), "mutants", "--package", "time-domain",
                   "--file", "crates/time-domain/src/leap/bundled.rs", "--test-workspace", "false",
                   "--cap-lints", "false", "--jobs", "10", "--jobserver", "true",
                   "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
                   "--error", policy["mutation_error_values"]["leap_bundled"][0],
                   "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                   "--output", "/work/mutation/leap_bundled"]
        MODULE.mutation_campaign_command(scope, tools, "leap_bundled", command,
                                        expected_jobs=10, expected_message_format="json", policy=policy)
        for flag, changed in [("--cap-lints", "true"), ("--jobs", "12"),
                              ("--baseline", "skip"), ("--timeout", "600"),
                              ("--error", "std::process::exit(0)"),
                              ("--file", "crates/time-domain/src/leap/generated.rs"),
                              ("--output", "/work/foreign")]:
            invalid = list(command)
            invalid[invalid.index(flag) + 1] = changed
            with self.subTest(flag=flag), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(scope, tools, "leap_bundled", invalid,
                                                expected_jobs=10, expected_message_format="json", policy=policy)


class EngineContractAdmissionTests(unittest.TestCase):
    """Constructor evidence cannot borrow engine runtime or historical gates."""

    def fixture(self):
        scope = MODULE.Scope("nevrotick-engine", (("library", "crates/nevrotick-engine/src/lib.rs"),),
                             "policy/engine-analysis.json")
        policy = {
            "schema_version": 1, "admission_status": "admitted-engine-external-contracts-only",
            "package": "nevrotick-engine", **MODULE.CONTRACT_PRODUCT_BINDING,
            "critical_files": ["crates/nevrotick-engine/src/lib.rs"],
            "tool_admission_profile": "stable-core", "contracts_wall_seconds": 300,
            "jobs": 6, "test_threads": 1, "mutation_equivalences": [], "mutation_error_values": {},
        }
        return scope, policy

    def test_engine_inert_contract_role_is_explicit_and_library_only(self):
        scope, policy = self.fixture()
        args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core")
        self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
        for lane in ("coverage", "mutation", "enumerate", "fuzz", "all", None):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)
        for changed in (
                MODULE.Scope("time-domain", scope.subjects, scope.policy),
                MODULE.Scope(scope.package, scope.subjects, "policy/other.json"),
                MODULE.Scope(scope.package, (*scope.subjects, ("cli", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs")), scope.policy),
                MODULE.Scope(scope.package, list(scope.subjects), scope.policy)):
            with self.subTest(scope=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, changed, policy)

    def test_engine_role_rejects_every_changed_missing_extra_or_aliased_field(self):
        scope, policy = self.fixture()
        args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core")
        for key, value in policy.items():
            choices = [None, True]
            if type(value) is int:
                choices.extend([float(value), value + 1])
            elif type(value) is list:
                choices.append(["foreign"])
            elif type(value) is dict:
                choices.append({"library": ["std::process::exit(0)"]})
            else:
                choices.append("foreign")
            for changed in choices:
                with self.subTest(key=key, changed=changed), self.assertRaises(MODULE.Incomplete):
                    MODULE.admission_profile(args, scope, {**policy, key: changed})
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admission_profile(args, scope, {**policy, "runtime_execution": True})

    def test_engine_contract_role_has_no_full_profile_fallback(self):
        scope, policy = self.fixture()
        for profile in (None, "full", "stable-engine", ""):
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane="contracts", tool_admission_profile=profile),
                                         scope, {**policy, "tool_admission_profile": profile})

    def test_engine_session_checks_actual_product_count_before_tools(self):
        # Synthetic reader mechanics: current checkout bytes are deliberately not
        # the historical admitted contract candidate. Keep its count/digest guard.
        count = MODULE.CONTRACT_PRODUCT_BINDING["source_file_count"]
        healthy = {f"synthetic/{index}.rs": "a" * 64 for index in range(count)}
        binding = {"source_file_count": count, "candidate_sha256": MODULE.canonical_sha256(healthy)}
        with patch.object(MODULE, "CONTRACT_PRODUCT_BINDING", binding):
            scope, policy = self.fixture()
            with tempfile.TemporaryDirectory() as temporary:
                parent = Path(temporary).resolve()
                root = parent / "checkout"
                root.mkdir()
                args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core", output_dir=parent / "output")
                for changed_count in (count - 1, count, count + 1):
                    identity = dict(healthy)
                    if changed_count < count:
                        identity.pop(next(iter(identity)))
                    elif changed_count > count:
                        identity["unexpected.rs"] = "a" * 64
                    expected = "healthy engine reached proofs" if changed_count == count else "complete contract candidate"
                    with (self.subTest(count=changed_count),
                          patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)),
                          patch.object(MODULE, "admit_mutation_equivalences",
                                       side_effect=MODULE.Incomplete("healthy engine reached proofs")) as proofs,
                          patch.object(MODULE.ANALYSIS, "admit") as tools,
                          self.assertRaisesRegex(MODULE.Incomplete, expected)):
                        MODULE.Session(root, args, {}, scope, policy, ())
                    self.assertEqual(proofs.call_count, int(changed_count == count))
                    tools.assert_not_called()
                    self.assertFalse(args.output_dir.exists())

    def test_same_count_replaced_product_cannot_reach_contract_tools(self):
        scope, policy = self.fixture()
        identity = {f"forged/{index}.rs": "a" * 64 for index in range(policy["source_file_count"])}
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary).resolve(); root = parent / "checkout"; root.mkdir()
            args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core", output_dir=parent / "output")
            with (patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)),
                  patch.object(MODULE, "admit_mutation_equivalences",
                               side_effect=MODULE.Incomplete("wrong candidate reached later admission")) as later,
                  patch.object(MODULE.ANALYSIS, "admit") as tools,
                  self.assertRaisesRegex(MODULE.Incomplete, "complete contract candidate")):
                MODULE.Session(root, args, {}, scope, policy, ())
            later.assert_not_called(); tools.assert_not_called()
            self.assertFalse(args.output_dir.exists())


class DomainContractAdmissionTests(unittest.TestCase):
    """Current ordinary domain contracts cannot alias a historical campaign role."""

    def fixture(self):
        scope = MODULE.Scope("time-domain", DomainLeapAdmissionTests.SUBJECTS, "policy/time-domain-analysis.json")
        policy = json.loads((ROOT / scope.policy).read_text())["external_contracts"]
        return scope, policy

    def test_exact_contract_role_and_scope_rejections(self):
        scope, policy = self.fixture()
        args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core")
        self.assertEqual(MODULE.admission_profile(args, scope, policy), "stable-core")
        for changed in (
                MODULE.Scope("time-core", scope.subjects, scope.policy),
                MODULE.Scope(scope.package, scope.subjects, "policy/other.json"),
                MODULE.Scope(scope.package, tuple(reversed(scope.subjects)), scope.policy),
                MODULE.Scope(scope.package, scope.subjects[:-1], scope.policy),
                MODULE.Scope(scope.package, list(scope.subjects), scope.policy)):
            with self.subTest(scope=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, changed, policy)
        for lane in ("coverage", "mutation", "enumerate", "fuzz", "all", None):
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(SimpleNamespace(lane=lane, tool_admission_profile="stable-core"), scope, policy)
        for key in policy:
            with self.subTest(missing=key), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, scope, {k: v for k, v in policy.items() if k != key})
        with self.assertRaises(MODULE.Incomplete):MODULE.admission_profile(args, scope, {**policy, "fuzz": True})

    def test_complete_current_snapshot_is_checked_before_tools_or_output(self):
        # Test the boundary with a declared synthetic snapshot; do not silently
        # call changed live source the old contract policy's healthy candidate.
        count = MODULE.CONTRACT_PRODUCT_BINDING["source_file_count"]
        healthy = {f"synthetic/{index}.rs": "a" * 64 for index in range(count)}
        binding = {"source_file_count": count, "candidate_sha256": MODULE.canonical_sha256(healthy)}
        scope, _ = self.fixture()
        with patch.object(MODULE, "CONTRACT_PRODUCT_BINDING", binding):
            policy = MODULE.domain_contract_facts()
            variants = [dict(healthy) for _ in range(4)]
            variants[1][next(iter(healthy))] = "0" * 64
            variants[2].pop(next(iter(healthy)))
            variants[3]["extra.rs"] = "0" * 64
            for index, identity in enumerate(variants):
                with self.subTest(index=index), tempfile.TemporaryDirectory() as temporary:
                    parent = Path(temporary).resolve(); root = parent / "checkout"; root.mkdir()
                    args = SimpleNamespace(lane="contracts", tool_admission_profile="stable-core", output_dir=parent / "output")
                    with (patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)),
                          patch.object(MODULE, "admit_mutation_equivalences", side_effect=MODULE.Incomplete("healthy reached later admission")) as later,
                          patch.object(MODULE.ANALYSIS, "admit") as tools,
                          self.assertRaisesRegex(MODULE.Incomplete, "healthy reached later admission" if index == 0 else "complete contract candidate")):
                        MODULE.Session(root, args, {}, scope, policy, ())
                    self.assertEqual(later.call_count, int(index == 0)); tools.assert_not_called()
                    self.assertFalse(args.output_dir.exists())


# Literal reviewed fault DTOs; no retained execution or historical qualification is reused.
_PHASE_STEP_REVIEWED_MUTANTS = {'PHASE-STEP-PROFILE-NORMAL-POSITIVE': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:228:42: '
                                                'replace > with >= in profile_valid',
                                        'package': 'time-core',
                                        'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                                        'function': {'function_name': 'profile_valid',
                                                     'return_type': '-> bool',
                                                     'span': {'start': {'line': 221, 'column': 1},
                                                              'end': {'line': 230, 'column': 2}}},
                                        'span': {'start': {'line': 228, 'column': 42},
                                                 'end': {'line': 228, 'column': 43}},
                                        'replacement': '>=',
                                        'genre': 'BinaryOperator'},
 'PHASE-STEP-DURATION-MIN-EQUALITY': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1378:49: '
                                              "replace < with <= in FixturePhasePlanOwner<'producer, "
                                              "'chart>::issue",
                                      'package': 'time-core',
                                      'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                                      'function': {'function_name': "FixturePhasePlanOwner<'producer, "
                                                                    "'chart>::issue",
                                                   'return_type': "-> Result<FixturePhaseProposal<'_>, "
                                                                  'FixturePhaseError>',
                                                   'span': {'start': {'line': 1335, 'column': 5},
                                                            'end': {'line': 1409, 'column': 6}}},
                                      'span': {'start': {'line': 1378, 'column': 49},
                                               'end': {'line': 1378, 'column': 50}},
                                      'replacement': '<=',
                                      'genre': 'BinaryOperator'},
 'PHASE-STEP-CLOCK-ORDER-RECHECK': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:2220:13: '
                                            'replace || with && in ControllerClockRelation::observe',
                                    'package': 'time-core',
                                    'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                                    'function': {'function_name': 'ControllerClockRelation::observe',
                                                 'return_type': '-> Result<Self, FixtureBaselineError>',
                                                 'span': {'start': {'line': 2209, 'column': 5},
                                                          'end': {'line': 2245, 'column': 6}}},
                                    'span': {'start': {'line': 2220, 'column': 13},
                                             'end': {'line': 2220, 'column': 15}},
                                    'replacement': '&&',
                                    'genre': 'BinaryOperator'},
 'PHASE-STEP-ANCHOR-ABS-SIGN': {'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:980:64: '
                                        'delete - in FrequencyAnchor::rebase',
                                'package': 'time-core',
                                'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                                'function': {'function_name': 'FrequencyAnchor::rebase',
                                             'return_type': '-> Result<Self, FixtureFrequencyNumericError>',
                                             'span': {'start': {'line': 960, 'column': 5},
                                                      'end': {'line': 987, 'column': 6}}},
                                'span': {'start': {'line': 980, 'column': 64},
                                         'end': {'line': 980, 'column': 65}},
                                'replacement': '',
                                'genre': 'UnaryOperator'},
 'PHASE-STEP-INTERVAL-ZERO-PRODUCT': {'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:1551:27: '
                                              'replace || with && in Interval::multiply',
                                      'package': 'time-core',
                                      'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                                      'function': {'function_name': 'Interval::multiply',
                                                   'return_type': '-> Result<Self, '
                                                                  'FixtureFrequencyNumericError>',
                                                   'span': {'start': {'line': 1543, 'column': 5},
                                                            'end': {'line': 1576, 'column': 6}}},
                                      'span': {'start': {'line': 1551, 'column': 27},
                                               'end': {'line': 1551, 'column': 29}},
                                      'replacement': '&&',
                                      'genre': 'BinaryOperator'},
 'PHASE-STEP-ISSUED-POSITIVE-DIRECTION': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs:708:38: '
                                                  'replace > with >= in operation_refusal',
                                          'package': 'time-core',
                                          'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs',
                                          'function': {'function_name': 'operation_refusal',
                                                       'return_type': '-> Option<FixtureStepError>',
                                                       'span': {'start': {'line': 687, 'column': 1},
                                                                'end': {'line': 733, 'column': 2}}},
                                          'span': {'start': {'line': 708, 'column': 38},
                                                   'end': {'line': 708, 'column': 39}},
                                          'replacement': '>=',
                                          'genre': 'BinaryOperator'},
 'PHASE-STEP-ISSUED-NEGATIVE-DIRECTION': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs:709:38: '
                                                  'replace < with <= in operation_refusal',
                                          'package': 'time-core',
                                          'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs',
                                          'function': {'function_name': 'operation_refusal',
                                                       'return_type': '-> Option<FixtureStepError>',
                                                       'span': {'start': {'line': 687, 'column': 1},
                                                                'end': {'line': 733, 'column': 2}}},
                                          'span': {'start': {'line': 709, 'column': 38},
                                                   'end': {'line': 709, 'column': 39}},
                                          'replacement': '<=',
                                          'genre': 'BinaryOperator'},
 'PHASE-STEP-ISSUED-POSITIVE-OVERSHOOT': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs:714:38: '
                                                  'replace > with >= in operation_refusal',
                                          'package': 'time-core',
                                          'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs',
                                          'function': {'function_name': 'operation_refusal',
                                                       'return_type': '-> Option<FixtureStepError>',
                                                       'span': {'start': {'line': 687, 'column': 1},
                                                                'end': {'line': 733, 'column': 2}}},
                                          'span': {'start': {'line': 714, 'column': 38},
                                                   'end': {'line': 714, 'column': 39}},
                                          'replacement': '>=',
                                          'genre': 'BinaryOperator'},
 'PHASE-STEP-ISSUED-NEGATIVE-OVERSHOOT': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs:715:42: '
                                                  'replace < with <= in operation_refusal',
                                          'package': 'time-core',
                                          'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs',
                                          'function': {'function_name': 'operation_refusal',
                                                       'return_type': '-> Option<FixtureStepError>',
                                                       'span': {'start': {'line': 687, 'column': 1},
                                                                'end': {'line': 733, 'column': 2}}},
                                          'span': {'start': {'line': 715, 'column': 42},
                                                   'end': {'line': 715, 'column': 43}},
                                          'replacement': '<=',
                                          'genre': 'BinaryOperator'},
 'PHASE-STEP-ADAPTIVE-NORMAL-POSITIVE': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs:95:40: '
                                                 'replace > with >= in adaptive_profile_valid',
                                         'package': 'time-core',
                                         'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs',
                                         'function': {'function_name': 'adaptive_profile_valid',
                                                      'return_type': '-> bool',
                                                      'span': {'start': {'line': 92, 'column': 1},
                                                               'end': {'line': 101, 'column': 2}}},
                                         'span': {'start': {'line': 95, 'column': 40},
                                                  'end': {'line': 95, 'column': 41}},
                                         'replacement': '>=',
                                         'genre': 'BinaryOperator'},
 'PHASE-STEP-ADAPTIVE-DISJOINT-MANTISSA': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs:263:49: '
                                                   'replace | with ^ in adaptive_ceil_nanoseconds',
                                           'package': 'time-core',
                                           'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs',
                                           'function': {'function_name': 'adaptive_ceil_nanoseconds',
                                                        'return_type': '-> Result<u64, FixturePhaseError>',
                                                        'span': {'start': {'line': 251, 'column': 1},
                                                                 'end': {'line': 289, 'column': 2}}},
                                           'span': {'start': {'line': 263, 'column': 49},
                                                    'end': {'line': 263, 'column': 50}},
                                           'replacement': '^',
                                           'genre': 'BinaryOperator'},
 'PHASE-STEP-ADAPTIVE-UNREACHABLE-CEILING': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs:285:32: '
                                                     'replace > with >= in adaptive_ceil_nanoseconds',
                                             'package': 'time-core',
                                             'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs',
                                             'function': {'function_name': 'adaptive_ceil_nanoseconds',
                                                          'return_type': '-> Result<u64, FixturePhaseError>',
                                                          'span': {'start': {'line': 251, 'column': 1},
                                                                   'end': {'line': 289, 'column': 2}}},
                                             'span': {'start': {'line': 285, 'column': 32},
                                                      'end': {'line': 285, 'column': 33}},
                                             'replacement': '>=',
                                             'genre': 'BinaryOperator'},
 'PHASE-STEP-SETTLED-BOUND-ABS-SIGN': {'name': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs:446:13: '
                                               'delete - in PhaseDebt::settled',
                                       'package': 'time-core',
                                       'file': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs',
                                       'function': {'function_name': 'PhaseDebt::settled',
                                                    'return_type': '-> Result<Self, '
                                                                   'FixturePhaseNumericError>',
                                                    'span': {'start': {'line': 390, 'column': 5},
                                                             'end': {'line': 458, 'column': 6}}},
                                       'span': {'start': {'line': 446, 'column': 13},
                                                'end': {'line': 446, 'column': 14}},
                                       'replacement': '',
                                       'genre': 'UnaryOperator'},
 'PHASE-STEP-STEPPED-BOUND-ABS-SIGN': {'name': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs:483:13: '
                                               'delete - in PhaseDebt::stepped',
                                       'package': 'time-core',
                                       'file': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs',
                                       'function': {'function_name': 'PhaseDebt::stepped',
                                                    'return_type': '-> Result<Self, '
                                                                   'FixturePhaseNumericError>',
                                                    'span': {'start': {'line': 460, 'column': 5},
                                                             'end': {'line': 488, 'column': 6}}},
                                       'span': {'start': {'line': 483, 'column': 13},
                                                'end': {'line': 483, 'column': 14}},
                                       'replacement': '',
                                       'genre': 'UnaryOperator'},
 'PHASE-STEP-POSITIVE-CLAMP-EQUALITY': {'name': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs:538:37: '
                                                'replace > with >= in PhaseDebt::proposal',
                                        'package': 'time-core',
                                        'file': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs',
                                        'function': {'function_name': 'PhaseDebt::proposal',
                                                     'return_type': '-> Result<PhaseRates, '
                                                                    'FixturePhaseNumericError>',
                                                     'span': {'start': {'line': 490, 'column': 5},
                                                              'end': {'line': 572, 'column': 6}}},
                                        'span': {'start': {'line': 538, 'column': 37},
                                                 'end': {'line': 538, 'column': 38}},
                                        'replacement': '>=',
                                        'genre': 'BinaryOperator'},
 'PHASE-STEP-NEGATIVE-CLAMP-EQUALITY': {'name': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs:540:28: '
                                                'replace < with <= in PhaseDebt::proposal',
                                        'package': 'time-core',
                                        'file': 'crates/time-core/src/regression/chart/roundoff/phase_plan.rs',
                                        'function': {'function_name': 'PhaseDebt::proposal',
                                                     'return_type': '-> Result<PhaseRates, '
                                                                    'FixturePhaseNumericError>',
                                                     'span': {'start': {'line': 490, 'column': 5},
                                                              'end': {'line': 572, 'column': 6}}},
                                        'span': {'start': {'line': 540, 'column': 28},
                                                 'end': {'line': 540, 'column': 29}},
                                        'replacement': '<=',
                                        'genre': 'BinaryOperator'}}



class PhaseStepEquivalenceAdmissionTests(unittest.TestCase):
    """Fresh-candidate review custody; no Rust executions or historical receipts."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.scope = MODULE.Scope("time-core", MODULE.PHASE_STEP_SUBJECTS, "policy/phase-step-analysis.json")
        self.assertEqual(set(_PHASE_STEP_REVIEWED_MUTANTS), MODULE.PHASE_STEP_EQUIVALENCE_IDS)
        names = {mutant["file"] for mutant in _PHASE_STEP_REVIEWED_MUTANTS.values()}
        for facts in MODULE.PHASE_STEP_EQUIVALENCE_REVIEWS.values():
            names.update(facts["dependencies"])
            names.update(facts.get("product_dependencies", {}))
        self.product_names = set(names)
        for name in names:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((ROOT / name).read_bytes())
        self.proof_path = self.root / MODULE.PHASE_STEP_EQUIVALENCE_PROOF
        self.proof_path.parent.mkdir(parents=True, exist_ok=True)
        proof = "\n".join(f"#### {identifier}\nIndependently reviewed ordinary source argument.\n"
                           for identifier in sorted(_PHASE_STEP_REVIEWED_MUTANTS))
        self.proof_path.write_text(proof + "bounded maintained task content\n" * 2400)
        identity = self.current_product()
        candidate = MODULE.canonical_sha256(identity)
        proof_sha = MODULE.PRODUCT.sha256(self.proof_path)
        self.policy = {**MODULE.phase_step_role_facts(), "source_file_count": len(identity),
            "candidate_sha256": candidate, "inputs_sha256": {
                **identity, MODULE.PHASE_STEP_EQUIVALENCE_PROOF: proof_sha},
            "mutation_inventories": {}, "test_inventories": {}, "scope": "Synthetic fresh review custody",
            "mutation_equivalences": []}
        for identifier, mutant in _PHASE_STEP_REVIEWED_MUTANTS.items():
            facts = MODULE.PHASE_STEP_EQUIVALENCE_REVIEWS[identifier]
            self.assertEqual(MODULE.canonical_sha256(mutant), facts["descriptor_sha256"])
            self.assertEqual(identity[mutant["file"]], facts["source_sha256"])
            self.policy["mutation_equivalences"].append({
                "id": identifier, "candidate_sha256": candidate, "source_sha256": facts["source_sha256"],
                "mutant": copy.deepcopy(mutant), "requirement": MODULE.PHASE_STEP_EQUIVALENCE_REQUIREMENT,
                "rationale": facts["rationale"], "proof": {
                    "path": MODULE.PHASE_STEP_EQUIVALENCE_PROOF, "sha256": proof_sha,
                    "section": identifier, "reviewer": MODULE.PHASE_STEP_EQUIVALENCE_REVIEWERS,
                    "verdict": "independently-justified-equivalent"}})

    def current_product(self):
        return {name: MODULE.PRODUCT.sha256(self.root / name) for name in self.product_names}

    def bind_candidate(self, policy):
        candidate = MODULE.canonical_sha256(self.current_product())
        policy["candidate_sha256"] = candidate
        for entry in policy["mutation_equivalences"]:
            entry["candidate_sha256"] = candidate

    def bind_proof(self, policy):
        digest = MODULE.PRODUCT.sha256(self.proof_path)
        policy["inputs_sha256"][MODULE.PHASE_STEP_EQUIVALENCE_PROOF] = digest
        for entry in policy["mutation_equivalences"]:
            entry["proof"]["sha256"] = digest

    def admit(self, policy=None, scope=None):
        with patch.object(MODULE.PRODUCT, "admit_product", side_effect=lambda _root: ({}, self.current_product())):
            return MODULE.admit_mutation_equivalences(self.root, scope or self.scope, policy or self.policy)

    def test_empty_and_complete_catalog_bind_the_actual_current_candidate(self):
        self.assertGreater(self.proof_path.stat().st_size, 65536)
        self.assertTrue(MODULE.phase_step_equivalences_valid(self.policy))
        MODULE.require_phase_step_role(self.scope, self.policy)
        self.assertEqual(self.admit(), self.policy["mutation_equivalences"])
        empty = {**self.policy, "mutation_equivalences": []}
        self.assertTrue(MODULE.phase_step_equivalences_valid(empty))
        MODULE.require_phase_step_role(self.scope, empty)
        self.assertEqual(self.admit(empty), [])
        # An independently changed test input requires a new candidate identity,
        # while the exact reviewed runtime sources/dependencies remain unchanged.
        path = self.root / "crates/time-core/tests/current_review_neighbor.rs"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("// Current synthetic test input, not an old execution.\n")
        self.product_names.add(path.relative_to(self.root).as_posix())
        changed = copy.deepcopy(self.policy)
        self.bind_candidate(changed)
        self.assertNotEqual(changed["candidate_sha256"], self.policy["candidate_sha256"])
        self.assertTrue(MODULE.phase_step_equivalences_valid(changed))
        self.assertEqual(self.admit(changed), changed["mutation_equivalences"])
        with self.assertRaises(MODULE.Incomplete):
            self.admit(self.policy)

    def test_partial_foreign_and_policy_invented_reviews_refuse(self):
        edits = [
            lambda p: p["mutation_equivalences"].pop(),
            lambda p: p["mutation_equivalences"].append(copy.deepcopy(p["mutation_equivalences"][0])),
            lambda p: p["mutation_equivalences"].__setitem__(1, copy.deepcopy(p["mutation_equivalences"][0])),
            lambda p: p["mutation_equivalences"][0].update(id="PHASE-STEP-UNREVIEWED"),
            lambda p: p.update(candidate_sha256=True),
            lambda p: p["mutation_equivalences"][0].update(candidate_sha256="a" * 64),
            lambda p: p["mutation_equivalences"][0].update(source_sha256="a" * 64),
            lambda p: p["mutation_equivalences"][0]["mutant"].update(replacement="unreviewed"),
            lambda p: p["mutation_equivalences"][0].update(rationale="Self-approved equivalence"),
            lambda p: p["mutation_equivalences"][0].update(requirement="foreign requirement"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(reviewer="unreviewed"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(verdict="accepted"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(section="PHASE-STEP-UNREVIEWED"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(sha256="a" * 64),
            lambda p: p["mutation_equivalences"][0]["proof"].update(path="tasks/foreign.md"),
            lambda p: p["mutation_equivalences"][0].update(skip_function=True),
        ]
        for edit in edits:
            policy = copy.deepcopy(self.policy)
            edit(policy)
            with self.subTest(edit=edit):
                self.assertFalse(MODULE.phase_step_equivalences_valid(policy))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.require_phase_step_role(self.scope, policy)
                with self.assertRaises(MODULE.Incomplete):
                    self.admit(policy)
        for value in (None, [], True):
            self.assertFalse(MODULE.phase_step_equivalences_valid(value))

    def test_requested_sign_reviews_bind_the_actual_requested_operand(self):
        """Independent source coordinates distinguish requested D from actual S/next zero."""
        filename = "crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs"
        source = (self.root / filename).read_text()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "0de5d97e268bc2833749b1ea0194014b250d51822e2febf00e33a5b8c04df811")
        rows = (
            ("PHASE-STEP-ISSUED-POSITIVE-DIRECTION", 708, 38, ">", ">=",
             "381cb2e3f952d6af6b2d5788024488b712ce157b45fe1413ffceed501d2aedd3",
             "981f6c193de8aabdd32e05724148d7bf344c36f9c25adbb0ec6e69c536b8a9d9"),
            ("PHASE-STEP-ISSUED-NEGATIVE-DIRECTION", 709, 38, "<", "<=",
             "1ce225d51195ebad9a84648575faeed02f92d66a92be4d97b1c4fa5f81fa6e9a",
             "195dddc709b2edf1ef0cae6309d71aa88b749d2acca0d2d0577f223ef28dddaa"),
            ("PHASE-STEP-ISSUED-POSITIVE-OVERSHOOT", 714, 38, ">", ">=",
             "76b9be08010417558ce0c2f5db28b6288ab3d4703fae9b5de300f94f99e87c63",
             "6b8a0aa54e5ff15b3e5c28c9ea592c8ca22da060f673ed9147160cad40dd1cf3"),
            ("PHASE-STEP-ISSUED-NEGATIVE-OVERSHOOT", 715, 42, "<", "<=",
             "c8d9a9a515a060778d8ad8539fbd8f0aabfa2dd1764398032ec8178f3baad108",
             "9ef0f6d808b34342b167bf5aa80bb77897e3ccb644413c23c8b9c3f896520fbb"),
        )

        def descriptor(line, column, original, replacement):
            return {"file": filename, "package": "time-core", "genre": "BinaryOperator",
                "name": f"{filename}:{line}:{column}: replace {original} with {replacement} in operation_refusal",
                "replacement": replacement,
                "span": {"start": {"line": line, "column": column},
                         "end": {"line": line, "column": column + 1}},
                "function": {"function_name": "operation_refusal", "return_type": "-> Option<FixtureStepError>",
                    "span": {"start": {"line": 687, "column": 1}, "end": {"line": 733, "column": 2}}}}

        for identifier, line, column, original, replacement, descriptor_sha, diff_sha in rows:
            with self.subTest(review=identifier):
                expected = descriptor(line, column, original, replacement)
                prefix = source.splitlines()[line - 1][:column - 1].rstrip()
                self.assertTrue(prefix.endswith("issued.requested.point()"))
                self.assertEqual(source.splitlines()[line - 1][column - 1], original)
                self.assertEqual(MODULE.canonical_sha256(expected), descriptor_sha)
                self.assertEqual(_PHASE_STEP_REVIEWED_MUTANTS[identifier], expected)
                facts = MODULE.PHASE_STEP_EQUIVALENCE_REVIEWS[identifier]
                self.assertEqual(facts["descriptor_sha256"], descriptor_sha)
                self.assertEqual(facts["diff_sha256"], diff_sha)
                self.assertEqual(hashlib.sha256(self.rendered_diff(expected).encode()).hexdigest(), diff_sha)
        wrong = (
            ("PHASE-STEP-ISSUED-NEGATIVE-DIRECTION", 709, 64, ">", ">=", "receipt.achieved"),
            ("PHASE-STEP-ISSUED-POSITIVE-OVERSHOOT", 714, 60, "<", "<=", "next.point()"),
            ("PHASE-STEP-ISSUED-NEGATIVE-OVERSHOOT", 715, 64, ">", ">=", "next.point()"),
        )
        for identifier, line, column, original, replacement, operand in wrong:
            with self.subTest(unreviewed_zero_capable_operand=operand, line=line):
                self.assertTrue(source.splitlines()[line - 1][:column - 1].rstrip().endswith(operand))
                policy = copy.deepcopy(self.policy)
                entry = next(entry for entry in policy["mutation_equivalences"] if entry["id"] == identifier)
                entry["mutant"] = descriptor(line, column, original, replacement)
                self.assertFalse(MODULE.phase_step_equivalences_valid(policy))
                with self.assertRaises(MODULE.Incomplete):
                    self.admit(policy)

    def test_reviewed_dependencies_must_match_pins_and_actual_product_bytes(self):
        dependencies = set()
        for facts in MODULE.PHASE_STEP_EQUIVALENCE_REVIEWS.values():
            dependencies.update(facts["dependencies"])
            dependencies.update(facts.get("product_dependencies", {}))
        for name in sorted(dependencies):
            policy = copy.deepcopy(self.policy)
            if any(name in facts["dependencies"] for facts in MODULE.PHASE_STEP_EQUIVALENCE_REVIEWS.values()):
                policy["inputs_sha256"][name] = "a" * 64
                self.assertFalse(MODULE.phase_step_equivalences_valid(policy))
            path = self.root / name
            original = path.read_bytes()
            path.write_bytes(original + b"// changed current dependency\n")
            coherent_candidate = copy.deepcopy(self.policy)
            self.bind_candidate(coherent_candidate)
            self.assertTrue(MODULE.phase_step_equivalences_valid(coherent_candidate))
            with self.subTest(dependency=name), self.assertRaises(MODULE.Incomplete):
                self.admit(coherent_candidate)
            path.write_bytes(original)

    def test_large_proof_exception_is_exact_role_path_level_and_bound(self):
        original = self.proof_path.read_bytes()
        for variant in ("body", "missing", "wrong_level", "duplicate", "oversized", "symlink"):
            policy = copy.deepcopy(self.policy)
            changed = original
            first = policy["mutation_equivalences"][0]["id"].encode()
            if variant == "body":
                changed += b"changed without re-admission\n"
            elif variant == "missing":
                changed = original.replace(b"#### " + first, b"#### FOREIGN-" + first, 1)
            elif variant == "wrong_level":
                changed = original.replace(b"#### " + first, b"## " + first, 1)
            elif variant == "duplicate":
                changed += b"\n#### " + first + b"\nDuplicate identity.\n"
            elif variant == "oversized":
                changed += b"x" * (2 * 1024 * 1024 + 1 - len(changed))
            if variant == "symlink":
                target = self.root / "tasks/linked-review.md"
                target.write_bytes(original)
                self.proof_path.unlink()
                self.proof_path.symlink_to(target)
            else:
                self.proof_path.write_bytes(changed)
                if variant != "body":
                    self.bind_proof(policy)
            with self.subTest(variant=variant), self.assertRaises(MODULE.Incomplete):
                self.admit(policy)
            if self.proof_path.is_symlink():
                self.proof_path.unlink()
            self.proof_path.write_bytes(original)
        exact = original + b"x" * (2 * 1024 * 1024 - len(original))
        self.proof_path.write_bytes(exact)
        policy = copy.deepcopy(self.policy)
        self.bind_proof(policy)
        self.assertEqual(self.admit(policy), policy["mutation_equivalences"])
        foreign = MODULE.Scope("time-core", MODULE.PHASE_STEP_SUBJECTS, "policy/foreign-analysis.json")
        with self.assertRaises(MODULE.Incomplete):
            self.admit(policy, foreign)

    def rendered_diff(self, mutant):
        source = (self.root / mutant["file"]).read_text()
        lines = source.splitlines(keepends=True)
        first, last = mutant["span"]["start"], mutant["span"]["end"]
        start = sum(map(len, lines[:first["line"] - 1])) + first["column"] - 1
        end = sum(map(len, lines[:last["line"] - 1])) + last["column"] - 1
        changed = source[:start] + mutant["replacement"] + " /* ~ changed by cargo-mutants ~ */" + source[end:]
        return "".join(difflib.unified_diff(source.splitlines(keepends=True), changed.splitlines(keepends=True),
            fromfile=mutant["file"], tofile=mutant["name"].split(": ", 1)[1], n=8))

    def inventory_fixture(self, policy):
        inventories = {}
        for entry in policy["mutation_equivalences"]:
            mutant = entry["mutant"]
            owner = next(name for name, filename in MODULE.PHASE_STEP_SUBJECTS if filename == mutant["file"])
            inventories.setdefault(owner, {})[mutant["name"]] = {
                **copy.deepcopy(mutant), "diff": self.rendered_diff(mutant)}
        policy["mutation_inventories"] = {
            name: {"count": 1, "sha256": "a" * 64} for name, _filename in MODULE.PHASE_STEP_SUBJECTS}
        for owner, rows in inventories.items():
            policy["mutation_inventories"][owner] = {"count": len(rows),
                "sha256": MODULE.canonical_sha256({
                    name: {key: value for key, value in row.items() if key != "diff"} for name, row in rows.items()})}
        return inventories

    def test_each_selected_owner_requires_only_its_own_exact_rendered_equivalences(self):
        policy = copy.deepcopy(self.policy)
        inventories = self.inventory_fixture(policy)
        for owner, rows in inventories.items():
            expected = {owner: policy["mutation_inventories"][owner]}
            with self.subTest(owner=owner):
                self.assertEqual(MODULE.admit_reviewed_mutation_inventories(
                    self.scope, policy, {owner: rows}), expected)
            for variant in ("changed_diff", "missing_diff", "missing_equivalent", "foreign_owner"):
                broken = copy.deepcopy(rows)
                key = next(iter(broken))
                if variant == "changed_diff":
                    broken[key]["diff"] += "\n"
                elif variant == "missing_diff":
                    del broken[key]["diff"]
                elif variant == "missing_equivalent":
                    del broken[key]
                else:
                    broken[key]["file"] = "crates/time-core/src/foreign.rs"
                bad = copy.deepcopy(policy)
                bad["mutation_inventories"][owner] = {"count": len(broken),
                    "sha256": MODULE.canonical_sha256({
                        name: {k: value for k, value in row.items() if k != "diff"} for name, row in broken.items()})}
                with self.subTest(owner=owner, variant=variant), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_reviewed_mutation_inventories(self.scope, bad, {owner: broken})

    def test_only_actual_misses_are_excluded_without_adding_a_kill(self):
        """Synthetic outcome ledger only; this executes no generated Rust specimen."""
        policy = copy.deepcopy(self.policy)
        inventories = self.inventory_fixture(policy)

        def phases(summary):
            command = ["cargo", "test", "--verbose", "--package=time-core@0.0.0", "--frozen"]
            build = {"phase": "Build", "duration": 0.25, "process_status": "Success",
                     "argv": [*command[:2], "--no-run", *command[2:]]}
            test = {"phase": "Test", "duration": 0.5, "process_status": "Success", "argv": command}
            if summary == "Unviable":
                return [{**build, "process_status": {"Failure": 101}}]
            if summary == "CaughtMutant":
                test["process_status"] = {"Failure": 101}
            return [build, test]

        def report(rows):
            counts = [row["summary"] for row in rows[1:]]
            return {"cargo_mutants_version": "27.1.0", "outcomes": rows, "total_mutants": len(rows) - 1,
                "caught": counts.count("CaughtMutant"), "missed": counts.count("MissedMutant"),
                "unviable": counts.count("Unviable"), "timeout": counts.count("Timeout"), "success": 0}

        for owner, inventory in inventories.items():
            filename = dict(MODULE.PHASE_STEP_SUBJECTS)[owner]
            expected = {name: {key: value for key, value in row.items() if key != "diff"}
                        for name, row in inventory.items()}
            equivalent = [entry for entry in policy["mutation_equivalences"]
                          if entry["mutant"]["file"] == filename]
            neighbor = copy.deepcopy(next(iter(expected.values())))
            neighbor["name"] += " [synthetic caught accounting neighbor]"
            expected[neighbor["name"]] = neighbor
            rows = [{"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success")},
                    {"scenario": {"Mutant": neighbor}, "summary": "CaughtMutant",
                     "phase_results": phases("CaughtMutant")}]
            rows.extend({"scenario": {"Mutant": entry["mutant"]}, "summary": "MissedMutant",
                         "phase_results": phases("MissedMutant")} for entry in equivalent)
            result = MODULE.mutation_counts(self.scope, report(rows), expected, filename,
                                            native_exit_code=2, equivalences=equivalent)
            self.assertEqual((result["killed"], result["survived"], result["excluded"]),
                             (1, len(equivalent), len(equivalent)))
            self.assertEqual(result["adjusted_denominator"], 1)
            self.assertTrue(result["gate_passed"])
            for summary in ("CaughtMutant", "Unviable", "Timeout"):
                wrong = copy.deepcopy(rows)
                wrong[2].update(summary=summary, phase_results=phases(summary))
                with self.subTest(owner=owner, unreviewable_outcome=summary), self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_counts(self.scope, report(wrong), expected, filename,
                        native_exit_code=2 if len(equivalent) > 1 else 0, equivalences=equivalent)


class EngineKeEquivalenceAdmissionTests(unittest.TestCase):
    """Synthetic admission controls; no product equivalence or kill is asserted."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.filename = dict(MODULE.ENGINE_KE_SUBJECTS)["ke_ledger"]
        source = self.root / self.filename
        source.parent.mkdir(parents=True)
        source.write_text("Synthetic checked caller, never compiled.\n")
        self.identity = {self.filename: MODULE.PRODUCT.sha256(source), "another-invariant-owner": "a" * 64}
        self.candidate = MODULE.canonical_sha256(self.identity)
        proof = self.root / MODULE.ENGINE_KE_EQUIVALENCE_PROOF
        proof.parent.mkdir()
        proof.write_text("Synthetic review only.\n" + "".join("## " + name + "\n" for name in sorted(MODULE.ENGINE_KE_EQUIVALENCE_IDS)))
        self.proof_sha = MODULE.PRODUCT.sha256(proof)
        self.catalog, entries = {}, []
        for index, name in enumerate(sorted(MODULE.ENGINE_KE_EQUIVALENCE_IDS)):
            mutant = {"name": name, "package": "nevrotick-engine", "file": self.filename,
                "function": {"function_name": "synthetic_checked_caller", "return_type": "-> bool",
                    "span": {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 2}}},
                "span": {"start": {"line": 1, "column": index + 1}, "end": {"line": 1, "column": index + 2}},
                "replacement": "&&", "genre": "BinaryOperator"}
            rationale = "Synthetic independently reviewed caller argument " + name
            self.catalog[name] = (self.identity[self.filename], MODULE.canonical_sha256(mutant), rationale)
            entries.append({"id": name, "candidate_sha256": self.candidate,
                "source_sha256": self.identity[self.filename], "mutant": mutant,
                "requirement": MODULE.ENGINE_KE_EQUIVALENCE_REQUIREMENT, "rationale": rationale,
                "proof": {"path": MODULE.ENGINE_KE_EQUIVALENCE_PROOF, "sha256": self.proof_sha,
                    "section": name, "reviewer": MODULE.ENGINE_KE_EQUIVALENCE_REVIEWERS,
                    "verdict": "independently-justified-equivalent"}})
        self.policy = {**MODULE.engine_ke_role_facts(), "binding": {"candidate_sha256": self.candidate},
            "campaigns_admitted": False, "mutation_inventories": {}, "mutation_equivalences": entries}
        self.scope = MODULE.Scope("nevrotick-engine", MODULE.ENGINE_KE_SUBJECTS, MODULE.ENGINE_KE_POLICY)

    @contextmanager
    def reviewed(self):
        """Substitute synthetic code-owned approvals, never Main policy or source."""
        with patch.multiple(MODULE, ENGINE_KE_EQUIVALENCE_CANDIDATE=self.candidate,
                ENGINE_KE_EQUIVALENCE_PROOF_SHA256=self.proof_sha, ENGINE_KE_EQUIVALENCE_REVIEWS=self.catalog):
            yield

    def test_preparatory_catalog_refuses_nonempty_and_preserves_empty_default(self):
        self.assertFalse(MODULE.engine_ke_equivalences_valid(self.policy))
        empty = copy.deepcopy(self.policy)
        empty["mutation_equivalences"] = []
        self.assertTrue(MODULE.engine_ke_equivalences_valid(empty))
        MODULE.require_engine_ke_role(self.scope, empty)

    def test_owned_contract_lane_requires_a_fixed_subject_and_stable_role(self):
        policy = {**self.policy, "mutation_equivalences": []}
        for subject in ("ke_process", "ke_job"):
            healthy = SimpleNamespace(lane="contracts", subject=subject, tool_admission_profile="stable-core")
            self.assertEqual(MODULE.admission_profile(healthy, self.scope, policy), "stable-core")
        for subject in (None, "all", "ke_core", "foreign", [], True):
            args = SimpleNamespace(lane="contracts", subject=subject, tool_admission_profile="stable-core")
            with self.subTest(subject=subject), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, self.scope, policy)
        for profile in (None, "full", "foreign"):
            args = SimpleNamespace(lane="contracts", subject="ke_process", tool_admission_profile=profile)
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, self.scope, {**policy, "tool_admission_profile": profile})
        for lane in ("baseline", "coverage", "mutation", "all"):
            args = SimpleNamespace(lane=lane, subject="all", tool_admission_profile="stable-core")
            with self.subTest(existing_lane=lane):
                self.assertEqual(MODULE.admission_profile(args, self.scope, policy), "stable-core")

    def test_six_worker_role_keeps_exact_admission_and_campaign_command(self):
        policy = {**self.policy, "mutation_jobs": 6, "mutation_equivalences": [],
                  "campaigns_admitted": True,
                  "mutation_inventories": {name: {"count": 1, "sha256": "a" * 64}
                                           for name, _path in MODULE.ENGINE_KE_SUBJECTS}}
        args = SimpleNamespace(lane="mutation", subject="ke_core", tool_admission_profile="stable-core")
        tools = Path("/owned/synthetic-tools")
        command = [str(tools / "cargo-mutants"), "mutants", "--package", "nevrotick-engine",
            "--file", "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs",
            "--test-workspace", "false", "--cap-lints", "false", "--jobs", "6",
            "--jobserver", "true", "--jobserver-tasks", "6", "--cargo-arg=--message-format=json",
            "--features", "ke-association-prototype", "--baseline", "run", "--build-timeout", "90",
            "--timeout", "60", "--output", "/work/mutation/ke_core"]
        self.assertEqual(MODULE.admission_profile(args, self.scope, policy), "stable-core")
        MODULE.mutation_campaign_command(self.scope, tools, "ke_core", command,
            expected_jobs=6, expected_message_format="json", policy=policy)
        for workers in (8, 10):
            bad = {**policy, "mutation_jobs": workers}
            with self.subTest(workers=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.admission_profile(args, self.scope, bad)
            substituted = command.copy()
            for flag in ("--jobs", "--jobserver-tasks"):
                substituted[substituted.index(flag) + 1] = str(workers)
            with self.subTest(coherent_foreign_command=workers), self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_campaign_command(self.scope, tools, "ke_core", substituted,
                    expected_jobs=workers, expected_message_format="json", policy=bad)

    def test_exact_complete_review_catalog_still_requires_generic_live_custody(self):
        self.assertEqual(MODULE.ENGINE_KE_EQUIVALENCE_IDS, frozenset({
            'KE-RETURN-POOL-CAPACITY-OR',
            'KE-RETURN-POOL-BYTES-OR',
            'KE-SEND-PREDECESSOR-VALUE-OR',
            'KE-SEND-ARCHIVE-VALUE-OR',
            'KE-SEND-ARCHIVE-CAPACITY-OR',
            'KE-SEND-ARCHIVE-BYTES-OR',
            'KE-SEND-ARCHIVE-BYTES-EQUALITY',
            'KE-SEND-ARCHIVE-BYTES-INCLUSIVE',
            'KE-RECEIVE-STABLE-SLOT-OR',
        }))
        self.assertEqual(len(self.policy["mutation_equivalences"]), 9)
        with self.reviewed(), patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, self.identity)):
            self.assertTrue(MODULE.engine_ke_equivalences_valid(self.policy))
            MODULE.require_engine_ke_role(self.scope, self.policy)
            self.assertEqual(MODULE.admit_mutation_equivalences(self.root, self.scope, self.policy),
                             self.policy["mutation_equivalences"])
            for relative in (self.filename, MODULE.ENGINE_KE_EQUIVALENCE_PROOF):
                path = self.root / relative
                original = path.read_bytes()
                path.write_bytes(original + b"changed\n")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_mutation_equivalences(self.root, self.scope, self.policy)
                path.write_bytes(original)
            changed = {**self.identity, "another-invariant-owner": "b" * 64}
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, changed)), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_mutation_equivalences(self.root, self.scope, self.policy)

    def test_neighbors_subsets_and_policy_controlled_approval_refuse(self):
        edits = [lambda p: p["binding"].update(candidate_sha256="f" * 64),
            lambda p: p["mutation_equivalences"].pop(),
            lambda p: p["mutation_equivalences"].append(copy.deepcopy(p["mutation_equivalences"][0])),
            lambda p: p.update(mutation_equivalences=[entry for entry in p["mutation_equivalences"]
                if entry["id"] != "KE-SEND-PREDECESSOR-VALUE-OR"]),
            lambda p: p["mutation_equivalences"].append({**copy.deepcopy(p["mutation_equivalences"][0]),
                "id": "UNREVIEWED-TENTH"}),
            lambda p: p["mutation_equivalences"][0].update(id="UNREVIEWED"),
            lambda p: p["mutation_equivalences"][0].update(candidate_sha256="f" * 64),
            lambda p: p["mutation_equivalences"][0].update(source_sha256="f" * 64),
            lambda p: p["mutation_equivalences"][0].update(rationale="Policy invented an approval"),
            lambda p: p["mutation_equivalences"][0]["mutant"].update(replacement="||"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(reviewer="self-approved"),
            lambda p: p["mutation_equivalences"][0]["proof"].update(sha256="f" * 64),
            lambda p: p["mutation_equivalences"][0]["proof"].update(path="tasks/foreign.md"),
            lambda p: p["mutation_equivalences"][0].update(skip_function=True)]
        with self.reviewed():
            for edit in edits:
                policy = copy.deepcopy(self.policy)
                edit(policy)
                self.assertFalse(MODULE.engine_ke_equivalences_valid(policy))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.require_engine_ke_role(self.scope, policy)


if __name__ == "__main__":
    unittest.main()
