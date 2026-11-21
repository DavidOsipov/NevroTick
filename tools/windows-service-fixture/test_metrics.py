"""Independent fixture metric controls; these fixtures never invoke analyzers."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("fixture_metrics_control", Path(__file__).with_name("metrics.py"))
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)


def descriptor(filename="src/host.rs", *, name="host guard"):
    """Author a complete native descriptor independently of the inventory parser."""
    span = {"start": {"line": 14, "column": 1}, "end": {"line": 18, "column": 2}}
    return {"name": name, "package": "nevrotick-windows-service-fixture", "file": filename,
            "function": {"function_name": "owner_guard", "return_type": "-> bool", "span": span},
            "span": {"start": {"line": 16, "column": 5}, "end": {"line": 16, "column": 9}},
            "replacement": "false", "genre": "FnValue"}


def inventories():
    """Keep all three source subjects explicit in every fixture."""
    return {"host": {"host guard": descriptor()},
            "error": {"error source": descriptor("src/error.rs", name="error source")},
            "diagnostics": {"diagnostics output": descriptor("src/diagnostics.rs", name="diagnostics output")}}


def execution(names=("healthy_owner", "failed_cleanup")):
    """Native test identities and zero-skipped summaries with independent facts."""
    return "".join(f"test {name} ... ok\n" for name in names) + (
        f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n")


def phases(summary):
    """Literal pinned Cargo phase grammar, distinct from assertion evidence."""
    result = []
    for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
        command = ["cargo", "test", *(["--no-run"] if phase == "Build" else []), "--verbose",
                   "--package=nevrotick-windows-service-fixture@0.0.0", "--message-format=json", "--frozen"]
        failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
        result.append({"phase": phase, "argv": command, "process_status": {"Failure": 101} if failed else "Success",
                       "duration": 0.01})
    return result


def outcomes(summary="CaughtMutant"):
    """One actual-format native outcome with exactly one ordinary baseline."""
    row = {"scenario": {"Mutant": descriptor()}, "summary": summary,
           "phase_results": phases(summary), "log_path": "mutant.log"}
    report = {"cargo_mutants_version": "27.1.0", "total_mutants": 1,
              "caught": int(summary == "CaughtMutant"), "missed": int(summary == "MissedMutant"),
              "unviable": int(summary == "Unviable"), "timeout": int(summary == "Timeout"), "success": 0,
              "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success"),
                             "log_path": "baseline.log"}, row]}
    return report


def review_fixture():
    """Concrete independent review entries, never automatic generation credit."""
    reconciliation = {"functions_and_guards": ["owner_guard: readiness cannot precede acknowledgement"],
                      "derives_and_macros": ["Eq/Debug derive: compared against independent categories"],
                      "unsafe_and_cfg_omissions": ["No unsafe fn in these owners; native module is Windows-only"],
                      "independent_tests": ["healthy_owner", "failed_cleanup"], "remaining_gaps": []}
    return {"schema_version": 1, "candidate_sha256": "a" * 64,
            "inventories": M.inventory_summary(inventories()), "reviewer": "independent-fixture-review",
            "reconciliation": {name: copy.deepcopy(reconciliation) for name in ("host", "error", "diagnostics")}}


def equivalence_fixture(root):
    """Independent exact source/proof facts, including a large existing task."""
    source_name = "tools/windows-service-fixture/src/host.rs"
    source = root / source_name
    source.parent.mkdir(parents=True)
    source.write_text("fn owner_guard() -> bool { false }\n")
    proof = root / "tasks/windows11-integration.md"
    proof.parent.mkdir()
    proof.write_text("Existing maintained task context.\n" * 2200 +
                     "#### FX-EQ-001 — exact fault review\n"
                     "The helper domain, returned value, errors and effects are identical for every admitted input.\n"
                     "Counterexample boundary search and independent reviewer retained these assumptions.\n")
    entry = {"id": "FX-EQ-001", "candidate_sha256": "a" * 64,
             "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(), "mutant": descriptor(),
             "requirement": "CHECK-25", "rationale": "Exact helper-boundary equivalence with reviewed effect preservation.",
             "proof": {"path": "tasks/windows11-integration.md", "sha256": hashlib.sha256(proof.read_bytes()).hexdigest(),
                       "section": "FX-EQ-001", "reviewer": "independent-reviewer", "verdict": "independently-justified-equivalent"}}
    review = {"schema_version": 1, "candidate_sha256": "a" * 64,
              "inventories": M.inventory_summary(inventories()), "mutation_equivalences": [entry]}
    return review, {source_name: entry["source_sha256"]}


def retirement_fixture(work):
    """Independent base/variant bytes and completed real-format observations."""
    source, derivative = work / "source", work / "worker-variant-000"
    base_worker = source / M.WORKER
    base_worker.parent.mkdir(parents=True)
    base_worker.write_bytes(b"original\n")
    (source / "unrelated.rs").write_bytes(b"unchanged\n")
    import shutil
    shutil.copytree(source, derivative)
    (derivative / M.WORKER).write_bytes(b"variant\n")
    class Session:
        def check(self):
            if M.C.PRODUCT.inventory(self.source) != self.identity:
                raise M.Incomplete("immutable base drift")
    session = Session()
    session.work, session.source = work, source
    session.identity = M.C.PRODUCT.inventory(source)
    session.tests = ["healthy_owner"]
    identity = {**session.identity, M.WORKER: hashlib.sha256(b"variant\n").hexdigest()}
    raw = {"name": "exact pilot", "package": "single_file", "file": "engine_worker.rs"}
    clean = {"exit_code": 0, "failure": None, "output": "cleaned"}
    build = {"exit_code": 0, "failure": None, "output": "compiled"}
    test = {"exit_code": 101, "failure": None, "output": "thread 'healthy_owner' panicked at src/engine_worker_tests.rs:9:5:\nassertion failed\n"}
    row = {"raw": raw, "source_sha256": identity[M.WORKER], "variant_sha256": M.Q.canonical(identity),
           "Clean": clean, "Build": build, "Test": test, **M.replay_outcome(build, test, session.tests)}
    session.record = {"direct_inventory": {raw["name"]: raw}, "commands": [
        {"label": "worker:0:" + label, "argv": ["literal", label], "variant_sha256": M.Q.canonical(identity), **result}
        for label, result in (("Clean", clean), ("Build", build), ("Test", test))]}
    return session, derivative, identity, row


class FixtureMetricControls(unittest.TestCase):
    """Challenge false passes without installing or invoking any analysis tool."""

    def test_diagnostics_is_an_exact_required_metric_owner_with_bounded_attribution(self):
        """The new output boundary has its own gate and only its actual source/tests qualify."""
        self.assertEqual(M.SUBJECTS, {"host": "src/host.rs", "error": "src/error.rs",
                                     "diagnostics": "src/diagnostics.rs", "engine_worker": "src/engine_worker.rs"})
        item = descriptor("src/diagnostics.rs", name="diagnostics output guard")
        self.assertEqual(M.inventory_listing(json.dumps([item]), "diagnostics"), {item["name"]: item})
        admitted = {"extensions": {"cargo-mutants": Path("/selected/cargo-mutants")}}
        command = M.mutation_base_command(admitted, "diagnostics")
        self.assertEqual(command[command.index("--file") + 1], "src/diagnostics.rs")
        self.assertIn("--cap-lints", command)
        self.assertEqual(command[command.index("--cap-lints") + 1], "false")
        for filename in ("src/diagnostics.rs", "src/diagnostics_tests.rs"):
            for prefix in ("", "tools/windows-service-fixture/", "/work/source/tools/windows-service-fixture/"):
                self.assertTrue(M.selected_span(prefix + filename))
        for filename in ("../src/diagnostics.rs", "src/diagnostics.rs/foreign", "src/diagnostics_extra.rs",
                         "/foreign/src/diagnostics.rs", "src/../src/diagnostics_tests.rs"):
            self.assertFalse(M.selected_span(filename))

    def test_direct_worker_raw_identity_and_native_refusal_are_distinct(self):
        source = b"fn guard() -> bool {\n    true\n}\n"
        span = {"start": {"line": 2, "column": 5}, "end": {"line": 2, "column": 9}}
        row = descriptor("engine_worker.rs", name="engine_worker.rs:2:5: replace guard -> bool with false")
        row.update(package="single_file", span=span,
                   function={"function_name": "guard", "return_type": "-> bool",
                             "span": {"start": {"line": 1, "column": 1}, "end": {"line": 3, "column": 2}}})
        row["diff"] = ("--- engine_worker.rs\n+++ replace guard -> bool with false\n"
                       "@@ -1,3 +1,3 @@\n fn guard() -> bool {\n-    true\n"
                       "+    false /* ~ changed by cargo-mutants ~ */\n }\n")
        raw = json.dumps([row])
        checked = M.direct_inventory(raw, source)
        self.assertEqual(checked[row["name"]]["raw"], row)
        self.assertEqual(checked[row["name"]]["source"], source.replace(b"true", b"false /* ~ changed by cargo-mutants ~ */"))
        with self.assertRaises(M.Incomplete):
            M.inventory_listing(raw, "host")
        for change in ("empty", "duplicate", "missing", "foreign", "bool", "diff", "span"):
            bad = copy.deepcopy(row)
            if change == "missing": del bad["genre"]
            if change == "foreign": bad["file"] = "host.rs"
            if change == "bool": bad["span"]["start"]["line"] = True
            if change == "diff": bad["diff"] += "poison"
            if change == "span": bad["span"]["end"]["column"] = 900
            rows = [] if change == "empty" else [bad, bad] if change == "duplicate" else [bad]
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.direct_inventory(json.dumps(rows), source)

    def test_direct_replay_failure_causes_are_typed_and_source_attributed(self):
        build = {"exit_code": 0, "failure": None, "output": "compiled"}
        text = ("thread 'healthy_owner' panicked at src/engine_worker_tests.rs:12:5:\n"
                "assertion `left == right` failed\nprocess abort signal SIGABRT\n")
        test = {"exit_code": 101, "failure": None, "output": text}
        self.assertEqual(M.replay_outcome(build, test, ["healthy_owner"])["kind"], "killed")
        self.assertEqual(M.replay_outcome(build, {**test, "exit_code": 0, "output": execution()},
                                        ["healthy_owner", "failed_cleanup"])["kind"], "survived")
        for bad in ("process abort signal SIGABRT\n", text.replace("engine_worker_tests.rs", "foreign.rs"),
                    text.replace("healthy_owner", "unselected_test"), text + "No space left on device\n",
                    text.replace("assertion `left == right` failed", "arbitrary panic")):
            with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                M.replay_outcome(build, {**test, "output": bad}, ["healthy_owner"])
        for broken in ({**test, "exit_code": True}, {**test, "failure": "timeout"}, {**test, "exit_code": -6}):
            with self.subTest(broken=broken), self.assertRaises(M.Incomplete):
                M.replay_outcome(build, broken, ["healthy_owner"])
        diagnostic = {"reason": "compiler-message", "target": {"name": "nevrotick_windows_service_fixture"},
                      "message": {"level": "error", "message": "mismatched types",
                                  "spans": [{"is_primary": True, "file_name": "src/engine_worker.rs"}]}}
        refused = {"exit_code": 101, "failure": None, "output": json.dumps(diagnostic) + "\n"}
        row = M.replay_outcome(refused, None, ["healthy_owner"])
        self.assertEqual((row["kind"], row["kill_credit"]), ("compiler-unviable", 0))
        for filename in ("src/host.rs", "foreign.rs", "../src/engine_worker.rs"):
            diagnostic["message"]["spans"][0]["file_name"] = filename
            with self.subTest(filename=filename), self.assertRaises(M.Incomplete):
                M.replay_outcome({**refused, "output": json.dumps(diagnostic) + "\n"}, None, ["healthy_owner"])
        with self.assertRaises(M.Incomplete):
            M.replay_outcome({**refused, "output": "error[E0001] arbitrary refusal"}, None, ["healthy_owner"])

    def test_replay_clean_is_package_scoped_and_successful_build_artifacts_are_fresh(self):
        self.assertEqual(M.replay_clean_command(), ["/toolchain/bin/cargo", "clean", "--manifest-path",
                         "/work/source/tools/windows-service-fixture/Cargo.toml", "--target-dir", "/work/target",
                         "-p", "nevrotick-windows-service-fixture", "--frozen", "--offline"])
        artifact = {"reason": "compiler-artifact",
                    "package_id": "path+file:///work/source/tools/windows-service-fixture#nevrotick-windows-service-fixture@0.0.0",
                    "manifest_path": "/work/source/tools/windows-service-fixture/Cargo.toml",
                    "target": {"name": "nevrotick_windows_service_fixture", "kind": ["lib"],
                               "src_path": "/work/source/tools/windows-service-fixture/src/lib.rs"},
                    "profile": {"test": True}, "fresh": False}
        dependency = {"reason": "compiler-artifact", "package_id": "registry+https://crates.io#dep@1.0.0",
                      "target": {"name": "dep"}, "fresh": True}
        output = json.dumps(dependency) + "\n" + json.dumps(artifact) + "\n"
        self.assertEqual(M.fresh_fixture_build(output), 1)
        for bad in ("", json.dumps(dependency), json.dumps({**artifact, "fresh": True}),
                    json.dumps({**artifact, "fresh": 0}), json.dumps({**artifact, "fresh": None}),
                    json.dumps({**artifact, "package_id": "path+file:///foreign#nevrotick-windows-service-fixture@0.0.0"}),
                    json.dumps({**artifact, "target": {"name": "foreign"}}),
                    json.dumps({**artifact, "manifest_path": "/foreign/Cargo.toml"}),
                    json.dumps({**artifact, "profile": {"test": False}}),
                    json.dumps({**artifact, "profile": {"test": 1}}),
                    json.dumps({**artifact, "target": {**artifact["target"], "kind": ["bin"]}}),
                    json.dumps({**artifact, "target": {**artifact["target"], "src_path": "/foreign/lib.rs"}})):
            with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                M.fresh_fixture_build(bad)

    def test_initial_replay_target_is_absent_or_empty_without_manual_cargo_marker(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            target = work / "target"
            self.assertEqual(M.initial_replay_target(work)["state"], "absent")
            target.mkdir()
            self.assertEqual(M.initial_replay_target(work)["state"], "empty-directory")
            marker = target / "unrelated"
            marker.write_bytes(b"retained")
            with self.assertRaises(M.Incomplete): M.initial_replay_target(work)
            self.assertEqual(marker.read_bytes(), b"retained")
            marker.unlink()
            target.rmdir()
            target.symlink_to(work, target_is_directory=True)
            with self.assertRaises(M.Incomplete): M.initial_replay_target(work)
            target.unlink()
            target.write_bytes(b"not a directory")
            with self.assertRaises(M.Incomplete): M.initial_replay_target(work)
            self.assertEqual(target.read_bytes(), b"not a directory")

    def test_worker_replay_cleans_each_source_before_build_and_closes_after_failed_clean(self):
        """Drive the actual pilot coordinator with command observations, never Cargo."""
        for failed_clean, cached_variant, cached_closing in ((False, False, False), (True, False, False),
                                                           (False, True, False), (False, False, True)):
            with self.subTest(failed_clean=failed_clean, cached_variant=cached_variant, cached_closing=cached_closing), \
                    tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                source = work / "source"
                worker = source / M.WORKER
                worker.parent.mkdir(parents=True)
                worker.write_bytes(b"original\n")
                raw = {"name": "exact pilot"}
                inventory = {"exact pilot": {"raw": raw, "source": b"variant\n",
                             "source_sha256": hashlib.sha256(b"variant\n").hexdigest()}}
                summary = M.direct_summary(inventory)
                reconciliation = review_fixture()["reconciliation"]["host"]
                events = []
                class Session:
                    def receipt(self, path):
                        if path == "inventory":
                            return {"status": "passed", "lane": "worker-enumerate", "bookends_passed": True,
                                    "export_complete": True, "graph_binding": self.binding,
                                    "direct_schema": M.DIRECT_SCHEMA, "direct_summary": summary,
                                    "direct_inventory": {"exact pilot": raw}}
                        return {"schema": M.DIRECT_SCHEMA, "candidate_sha256": "a" * 64,
                                "summary": summary, "reviewer": "independent", "reconciliation": reconciliation}
                    def run(self, label, command, seconds):
                        result = observe(label, command)
                        M.successful(result)
                        return result
                session = Session()
                session.args = type("Args", (), {"lane": "worker-replay-pilot", "direct_inventory": "inventory",
                    "reviewed_inventory": "review", "pilot_mutant": "exact pilot"})()
                session.work, session.source, session.tests = work, source, ["healthy_owner"]
                session.identity = {M.WORKER: hashlib.sha256(b"original\n").hexdigest()}
                session.admitted = {"extensions": {"cargo-mutants": Path("/admitted/cargo-mutants")}}
                session.record, session.binding = {"candidate_sha256": "a" * 64}, {"bound": "graph"}
                def observe(label, command):
                    events.append(label)
                    if label.endswith(":Clean"):
                        self.assertEqual(command, M.replay_clean_command())
                    result = {"exit_code": 0, "failure": None, "output": "clean"}
                    if label == "worker:0:Clean" and failed_clean:
                        result.update(exit_code=101, output="clean refused")
                    elif label.endswith(":Build"):
                        cached = (cached_variant and label == "worker:0:Build") or (cached_closing and label == "worker-closing:Build")
                        result["output"] = json.dumps({"reason": "compiler-artifact",
                            "package_id": "path+file:///work/source/tools/windows-service-fixture#nevrotick-windows-service-fixture@0.0.0",
                            "manifest_path": "/work/source/tools/windows-service-fixture/Cargo.toml",
                            "target": {"name": "nevrotick_windows_service_fixture", "kind": ["lib"],
                                       "src_path": "/work/source/tools/windows-service-fixture/src/lib.rs"},
                            "profile": {"test": True}, "fresh": cached}) + "\n"
                    elif label == "worker:0:Test":
                        result.update(exit_code=101, output="thread 'healthy_owner' panicked at src/engine_worker_tests.rs:9:5:\nassertion failed\n")
                    elif label.endswith(":Test"):
                        result["output"] = execution(("healthy_owner",))
                    elif label.endswith(":doc"):
                        result["output"] = "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\n"
                    return result
                with patch.object(M, "direct_inventory", return_value=inventory), \
                        patch.object(M, "check_variant"), \
                        patch.object(M, "retire_variant") as retirement, \
                        patch.object(M.Q, "lane_commands", return_value=[["doc"]]), \
                        patch.object(M, "replay_command", side_effect=lambda sess, src, identity, label, command, seconds:
                                     observe(label, command)):
                    if failed_clean or cached_variant or cached_closing:
                        with self.assertRaises(M.Incomplete): M.execute_worker(session)
                    else:
                        M.execute_worker(session)
                        self.assertTrue(session.record["pilot_passed"])
                        self.assertFalse(session.record["replay_score"]["gate_passed"])
                        self.assertIn("Clean", session.record["replay_rows"]["exact pilot"])
                self.assertEqual(retirement.call_count, int(not failed_clean and not cached_variant))
                self.assertEqual(events[:4], ["worker-direct-generated", "worker-baseline:Build",
                                             "worker-baseline:Test", "worker-baseline:doc"])
                self.assertLess(events.index("worker:0:Clean"), events.index("worker-closing:Clean"))
                if failed_clean:
                    self.assertNotIn("worker:0:Build", events)
                else:
                    self.assertLess(events.index("worker:0:Clean"), events.index("worker:0:Build"))
                if cached_variant:
                    self.assertNotIn("worker:0:Test", events)
                self.assertEqual(events[-2:] if cached_closing else events[-3:],
                                 ["worker-closing:Clean", "worker-closing:Build"] if cached_closing else
                                 ["worker-closing:Clean", "worker-closing:Build", "worker-closing:Test"])
                self.assertEqual(worker.read_bytes(), b"original\n")

    def test_direct_commands_use_writable_target_and_full_fixture_roster(self):
        build, test = M.replay_commands()
        self.assertIn("--no-run", build)
        self.assertNotIn("--no-run", test)
        for command in (build, test):
            self.assertEqual(command[command.index("--target-dir") + 1], "/work/target")
            self.assertEqual(command[command.index("--jobs") + 1], "1")
            self.assertIn("--all-targets", command)
            self.assertIn("--frozen", command)
            self.assertIn("--offline", command)
        self.assertEqual(test[-3:], ["--", "--test-threads=1", "--nocapture"])

    def test_direct_score_partial_empty_unviable_and_boolean_counters_never_pass(self):
        inventory = {"one": {}, "two": {}}
        killed = {"kind": "killed", "kill_credit": 1}
        pilot = M.replay_score({"one": killed}, inventory, complete=False)
        self.assertFalse(pilot["gate_passed"])
        self.assertIsNone(pilot["viable_killed_percent"])
        full = M.replay_score({"one": killed, "two": killed}, inventory, complete=True)
        self.assertTrue(full["gate_passed"])
        self.assertEqual((full["generated"], full["viable"], full["killed"], full["excluded"]), (2, 2, 2, 0))
        unviable = {"kind": "compiler-unviable", "kill_credit": 0}
        self.assertFalse(M.replay_score({"one": unviable, "two": unviable}, inventory, complete=True)["gate_passed"])
        for rows in ({}, {"one": killed}, {"foreign": killed},
                     {"one": {**killed, "kill_credit": True}, "two": killed},
                     {"one": {"kind": "timeout", "kill_credit": 0}, "two": killed}):
            with self.subTest(rows=rows), self.assertRaises(M.Incomplete):
                M.replay_score(rows, inventory, complete=True)

    def test_retired_variant_retains_exact_reconstructable_worker_and_original_base(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, derivative, identity, row = retirement_fixture(Path(temporary))
            result = M.retire_variant(session, derivative, identity, 0, row)
            self.assertFalse(derivative.exists())
            self.assertEqual((session.source / M.WORKER).read_bytes(), b"original\n")
            retained = session.work / result["path"]
            self.assertEqual((retained / "engine_worker.rs").read_bytes(), b"variant\n")
            manifest = json.loads((retained / "manifest.json").read_text())
            restored = dict(manifest["original_source_sha256"])
            restored[M.WORKER] = hashlib.sha256((retained / "engine_worker.rs").read_bytes()).hexdigest()
            self.assertEqual(restored, identity)
            self.assertEqual(manifest["variant_source_sha256"], identity)
            self.assertEqual(manifest["variant_sha256"], M.Q.canonical(restored))
            self.assertEqual(manifest["command_labels"], ["worker:0:Clean", "worker:0:Build", "worker:0:Test"])
            self.assertEqual(result["status"], "retired")

    def test_variant_retirement_refuses_unsafe_inputs_without_deleting_sources(self):
        for failure in ("base", "sibling", "missing-base", "extra", "source-drift", "failed-Q", "unclassified",
                        "missing-command", "worker-tamper", "manifest-tamper", "archive-extra", "bool-credit",
                        "archive-parent-link", "derivative-link"):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as temporary:
                session, derivative, identity, row = retirement_fixture(Path(temporary))
                selected = derivative
                sibling = session.work / "sibling"
                sibling.mkdir()
                (sibling / "sentinel").write_bytes(b"keep sibling")
                if failure == "base": selected = session.source
                if failure == "sibling": selected = sibling
                if failure == "missing-base": (session.source / "unrelated.rs").unlink()
                if failure == "extra": (derivative / "unexpected.rs").write_bytes(b"unknown")
                if failure == "source-drift": (derivative / M.WORKER).write_bytes(b"altered")
                if failure == "failed-Q": row["Test"]["failure"] = "resource stop"
                if failure == "unclassified": row["kind"] = "unknown"
                if failure == "bool-credit": row["kill_credit"] = True
                if failure == "missing-command": session.record["commands"].pop()
                if failure == "archive-parent-link": (session.work / "worker-replay").symlink_to(sibling, target_is_directory=True)
                if failure == "derivative-link":
                    derivative.rename(session.work / "saved-variant")
                    derivative.symlink_to(session.work / "saved-variant", target_is_directory=True)
                original_write = M.Q.write_new
                def write(path, value):
                    original_write(path, value)
                    if failure == "worker-tamper": (path.parent / "engine_worker.rs").write_bytes(b"altered archive")
                    if failure == "archive-extra": (path.parent / "unexpected").write_bytes(b"unknown")
                    if failure == "manifest-tamper":
                        value = json.loads(path.read_text())
                        value["extra"] = "unknown"
                        path.write_text(json.dumps(value))
                with patch.object(M.Q, "write_new", side_effect=write), self.assertRaises((M.Incomplete, ValueError)):
                    M.retire_variant(session, selected, identity, 0, row)
                self.assertTrue(derivative.exists())
                self.assertEqual((session.source / M.WORKER).read_bytes(), b"original\n")
                self.assertEqual((sibling / "sentinel").read_bytes(), b"keep sibling")

    def test_variant_retirement_deletion_failure_preserves_sparse_evidence_and_partial_tree(self):
        with tempfile.TemporaryDirectory() as temporary:
            session, derivative, identity, row = retirement_fixture(Path(temporary))
            sibling = session.work / "sibling"
            sibling.mkdir()
            (sibling / "sentinel").write_bytes(b"unrelated")
            def fail_delete(path):
                (path / "unrelated.rs").unlink()
                raise OSError("synthetic partial retirement failure")
            with patch.object(M.shutil, "rmtree", side_effect=fail_delete), self.assertRaises(OSError):
                M.retire_variant(session, derivative, identity, 0, row)
            self.assertTrue(derivative.exists())
            self.assertEqual(row["retention"]["status"], "incomplete-possibly-partial-derivative")
            self.assertIn("synthetic partial retirement failure", row["retention"]["retirement_error"])
            retained = session.work / row["retention"]["path"]
            manifest = json.loads((retained / "manifest.json").read_text())
            M.retained_variant(session, retained, manifest)
            self.assertEqual((retained / "engine_worker.rs").read_bytes(), b"variant\n")
            self.assertEqual((session.source / M.WORKER).read_bytes(), b"original\n")
            self.assertEqual((sibling / "sentinel").read_bytes(), b"unrelated")

    def test_variant_byte_bookends_reject_base_extra_file_and_wrong_restoration(self):
        with tempfile.TemporaryDirectory() as temporary:
            work = Path(temporary)
            original, derivative = work / "source", work / "variant"
            original.mkdir(); derivative.mkdir()
            (derivative / "file.rs").write_bytes(b"exact variant")
            identity = {"file.rs": hashlib.sha256(b"exact variant").hexdigest()}
            class Session:
                def check(self):
                    if self.drift:
                        raise M.Incomplete("immutable original drift")
            session = Session()
            session.work, session.source, session.drift = work, original, False
            with patch.object(M.C.PRODUCT, "inventory", side_effect=lambda path: {
                    p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in path.iterdir()}):
                M.check_variant(session, derivative, identity)
                for change in ("base", "extra", "restored", "original"):
                    with self.subTest(change=change), self.assertRaises(M.Incomplete):
                        if change == "base": M.check_variant(session, original, identity)
                        elif change == "extra":
                            (derivative / "foreign.rs").write_bytes(b"unexpected")
                            try: M.check_variant(session, derivative, identity)
                            finally: (derivative / "foreign.rs").unlink()
                        elif change == "restored":
                            (derivative / "file.rs").write_bytes(b"wrong original")
                            try: M.check_variant(session, derivative, identity)
                            finally: (derivative / "file.rs").write_bytes(b"exact variant")
                        else:
                            session.drift = True
                            M.check_variant(session, derivative, identity)

    def test_worker_coverage_cannot_borrow_three_native_owner_metrics(self):
        report = {"data": [{"files": [{"filename": "/work/source/" + M.Q.TOOL + "/" + filename,
                   "summary": {"lines": {"count": 100, "covered": 90}}}
                   for filename in M.SUBJECTS.values()]}]}
        self.assertEqual(M.metric_coverage(report)["engine_worker"]["line_percent"], 90)
        for bad in ("missing", "low", "duplicate", "foreign", "bool"):
            changed = copy.deepcopy(report)
            rows = changed["data"][0]["files"]
            if bad == "missing": rows.pop()
            if bad == "low": rows[-1]["summary"]["lines"]["covered"] = 89
            if bad == "duplicate": rows.append(copy.deepcopy(rows[-1]))
            if bad == "foreign": rows[-1]["filename"] = "/foreign/src/engine_worker.rs"
            if bad == "bool": rows[-1]["summary"]["lines"]["covered"] = True
            with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                M.metric_coverage(changed)

    def test_received_record_is_parsed_from_one_strict_owned_snapshot(self):
        self.assertEqual(M.decode_record(b'{"status":"passed","count":1}'), {"status": "passed", "count": 1})
        for data in (b'{"count":1,"count":2}', b'{"value":NaN}', b'{"value":1e999}',
                     b'[]', b'{"value":' + b'9' * 129 + b'}',
                     b'{"value":' + b'[' * 65 + b'0' + b']' * 65 + b'}'):
            with self.subTest(data=data), self.assertRaises(M.Incomplete):
                M.decode_record(data)

    def test_exact_coverage_owners_pass_at_boundary_and_retained_unrelated_data_is_not_a_gate(self):
        report = {"data": [{"files": [
            {"filename": "/work/source/tools/windows-service-fixture/src/host.rs",
             "summary": {"lines": {"count": 100, "covered": 90}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/error.rs",
             "summary": {"lines": {"count": 10, "covered": 10}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/diagnostics.rs",
             "summary": {"lines": {"count": 100, "covered": 95}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/engine_worker.rs",
             "summary": {"lines": {"count": 100, "covered": 95}}},
            {"filename": "/work/source/unrelated.rs", "summary": {"lines": {"count": 1000, "covered": 0}}}]}]}
        before = copy.deepcopy(report)
        result = M.metric_coverage(report)
        self.assertEqual(result["host"]["line_percent"], 90)
        self.assertEqual(result["error"]["line_percent"], 100)
        self.assertEqual(result["diagnostics"]["line_percent"], 95)
        self.assertEqual(report, before)
        for change in ("missing", "empty", "low", "duplicate", "foreign"):
            broken = copy.deepcopy(report)
            files = broken["data"][0]["files"]
            if change == "missing":
                del files[0]
            elif change == "empty":
                files[0]["summary"]["lines"] = {"count": 0, "covered": 0}
            elif change == "low":
                files[0]["summary"]["lines"]["covered"] = 89
            elif change == "duplicate":
                files.append(copy.deepcopy(files[0]))
            else:
                files[0]["filename"] = "/work/source/foreign/src/host.rs"
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.metric_coverage(broken)

    def test_diagnostics_cannot_borrow_coverage_or_inventory_from_other_owners(self):
        """A green host/error pair cannot hide a missing or weak diagnostics owner."""
        report = {"data": [{"files": [
            {"filename": "/work/source/tools/windows-service-fixture/src/host.rs",
             "summary": {"lines": {"count": 100, "covered": 100}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/error.rs",
             "summary": {"lines": {"count": 100, "covered": 100}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/diagnostics.rs",
             "summary": {"lines": {"count": 100, "covered": 90}}},
            {"filename": "/work/source/tools/windows-service-fixture/src/engine_worker.rs",
             "summary": {"lines": {"count": 100, "covered": 90}}}]}]}
        self.assertEqual(M.metric_coverage(report)["diagnostics"]["line_percent"], 90)
        for change in ("low", "missing", "duplicate", "foreign"):
            bad = copy.deepcopy(report)
            files = bad["data"][0]["files"]
            if change == "low":
                files[2]["summary"]["lines"]["covered"] = 89
            elif change == "missing":
                del files[2]
            elif change == "duplicate":
                files.append(copy.deepcopy(files[2]))
            else:
                files[2]["filename"] = "/foreign/src/diagnostics.rs"
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.metric_coverage(bad)
        rows = inventories()
        del rows["diagnostics"]
        with self.assertRaises(M.Incomplete):
            M.inventory_summary(rows)
        review = review_fixture()
        del review["reconciliation"]["diagnostics"]
        with self.assertRaises(M.Incomplete):
            M.receive_review(review, "a" * 64, M.inventory_summary(inventories()))

    def test_diagnostics_actual_assertion_and_compiler_sources_keep_distinct_credit(self):
        """Only selected diagnostics assertions can kill; compiler faults remain zero credit."""
        item = descriptor("src/diagnostics.rs", name="diagnostics output")
        expected = {item["name"]: item}
        text = ("test bounded_output ... FAILED\n\n---- bounded_output stdout ----\n"
                "thread 'bounded_output' panicked at src/diagnostics_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n\nfailures:\n")
        diagnostic = {"reason": "compiler-message", "target": {"name": "nevrotick_windows_service_fixture"},
                      "message": {"level": "error", "message": "mismatched types",
                                  "spans": [{"is_primary": True, "file_name": "src/diagnostics.rs"}]}}
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            for summary in ("CaughtMutant", "Unviable"):
                report = outcomes(summary)
                report["outcomes"][1]["scenario"]["Mutant"] = item
                (directory / "mutant.log").write_text(text if summary == "CaughtMutant" else json.dumps(diagnostic) + "\n")
                causes = M.mutation_causes(directory, report, ["bounded_output"])
                if summary == "CaughtMutant":
                    self.assertEqual(causes[item["name"]]["kind"], "behavioral-assertion")
                    self.assertEqual(causes[item["name"]]["causes"][0]["source"], "src/diagnostics_tests.rs")
                    result = M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs", defer_gate=True)
                    self.assertEqual((result["killed"], result["unviable"]), (1, 0))
                else:
                    self.assertEqual(causes[item["name"]]["kind"], "compiler-unviable")
                    self.assertEqual(causes[item["name"]]["kill_credit"], 0)
                    deferred = M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs", defer_gate=True)
                    self.assertEqual((deferred["viable"], deferred["killed"], deferred["unviable"]), (0, 0, 1))
                    self.assertIsNone(deferred["adjusted_viable_killed_percent"])
                    self.assertFalse(deferred["gate_passed"])
                    with self.assertRaisesRegex(M.Incomplete, "no viable denominator"):
                        M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs")
            report = outcomes("MissedMutant")
            report["outcomes"][1]["scenario"]["Mutant"] = item
            result = M.C.mutation_counts(M.SCOPE, report, expected, "src/diagnostics.rs",
                                         native_exit_code=2, defer_gate=True)
            self.assertFalse(result["gate_passed"])
            self.assertEqual(result["unresolved_survivors"], ["diagnostics output"])

    def test_native_test_names_counts_and_failures_cannot_be_substituted(self):
        self.assertEqual(M.executed_tests(execution()), ["failed_cleanup", "healthy_owner"])
        for text in ("", execution(()), execution().replace("0 ignored", "1 ignored"),
                     execution().replace("0 filtered out", "1 filtered out"),
                     execution().replace("healthy_owner ... ok", "healthy_owner ... FAILED"),
                     execution(("healthy_owner", "healthy_owner")),
                     execution().replace("2 passed", "3 passed")):
            with self.subTest(text=text), self.assertRaises(M.Incomplete):
                M.executed_tests(text)
        with self.assertRaisesRegex(M.Incomplete, "omits or adds"):
            M.executed_tests(execution(), ["healthy_owner"])
        docs = execution() + execution(("src/lib.rs - (line 9)",))
        self.assertEqual(len(M.executed_tests(docs, ["healthy_owner", "failed_cleanup"], allow_doctests=True)), 3)

    def test_command_failure_and_timeout_receive_no_metric_credit(self):
        for result in ({}, {"exit_code": True, "failure": None, "output": ""},
                       {"exit_code": 0, "failure": "wall timeout", "output": ""},
                       {"exit_code": 101, "failure": None, "output": "assertion failed"}):
            with self.subTest(result=result), self.assertRaises(M.Incomplete):
                M.successful(result, mutation=True)
        M.successful({"exit_code": 2, "failure": None, "output": "complete native missed report"}, mutation=True)
        with self.assertRaises(M.Incomplete):
            M.successful({"exit_code": 2, "failure": None, "output": ""})

    def test_complete_descriptor_inventory_is_exact_finite_and_unfiltered(self):
        item = descriptor()
        self.assertEqual(M.inventory_listing(json.dumps([item]), "host"), {item["name"]: item})
        for text in ("[]", "{}", "[NaN]", "[] trailing", '[{"file":"a","file":"b"}]',
                     json.dumps([item, item]), json.dumps([descriptor("src/error.rs")]),
                     json.dumps([{**item, "package": "time-core"}]),
                     json.dumps([{key: value for key, value in item.items() if key != "span"}]),
                     json.dumps([{**item, "function": None}])):
            with self.subTest(text=text), self.assertRaises(M.Incomplete):
                M.inventory_listing(text, "host")
        oversized = [descriptor(name=f"fault {index}") for index in range(513)]
        with self.assertRaises(M.Incomplete):
            M.inventory_listing(json.dumps(oversized), "host")
        with self.assertRaises(M.Incomplete):
            M.inventory_summary({"host": inventories()["host"]})

    def test_review_hashes_bind_all_descriptor_members_and_require_omission_reconciliation(self):
        rows = inventories()
        summaries = M.inventory_summary(rows)
        rows["host"]["host guard"]["diff"] = "rendered preview"
        self.assertEqual(M.inventory_summary(rows), summaries)
        rows["host"]["host guard"]["replacement"] = "true"
        self.assertNotEqual(M.inventory_summary(rows), summaries)
        review = review_fixture()
        self.assertIs(M.receive_review(review, "a" * 64, summaries), review)
        for change in ("candidate", "count", "hash", "reviewer", "omissions", "functions", "tests"):
            bad = copy.deepcopy(review)
            if change == "candidate":
                bad["candidate_sha256"] = "b" * 64
            elif change == "count":
                bad["inventories"]["host"]["count"] += 1
            elif change == "hash":
                bad["inventories"]["host"]["sha256"] = "b" * 64
            elif change == "reviewer":
                bad["reviewer"] = ""
            else:
                field = {"omissions": "unsafe_and_cfg_omissions", "functions": "functions_and_guards",
                         "tests": "independent_tests"}[change]
                bad["reconciliation"]["host"][field] = []
            with self.subTest(change=change), self.assertRaises(M.Incomplete):
                M.receive_review(bad, "a" * 64, summaries)

    def test_per_file_mutation_rejects_missing_outcomes_compiler_only_denominator_and_survivors(self):
        expected = inventories()["host"]
        report = outcomes()
        result = M.C.mutation_counts(M.SCOPE, report, expected, "src/host.rs")
        self.assertEqual((result["killed"], result["viable"], result["viable_killed_percent"]), (1, 1, 100))
        for bad in (outcomes("Unviable"), {**report, "total_mutants": 2},
                    {**report, "outcomes": report["outcomes"][:1]}):
            with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                M.C.mutation_counts(M.SCOPE, bad, expected, "src/host.rs")
        survived = M.C.mutation_counts(M.SCOPE, outcomes("MissedMutant"), expected, "src/host.rs",
                                      native_exit_code=2, defer_gate=True)
        self.assertFalse(survived["gate_passed"])
        self.assertEqual(survived["unresolved_survivors"], ["host guard"])

    def test_exact_equivalence_admission_binds_complete_inventory_source_and_existing_task_proof(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            before = copy.deepcopy(review)
            admitted = M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)
            self.assertEqual(admitted, review["mutation_equivalences"])
            self.assertEqual(review, before)
            self.assertIsNot(admitted, review["mutation_equivalences"])
            self.assertGreater((root / "tasks/windows11-integration.md").stat().st_size, 65536)
            for change in ("candidate", "source", "inventory", "bool-count", "unknown", "replacement", "duplicate-id",
                           "duplicate-fault", "proof-hash", "proof-section", "no-proof", "no-reviewer", "no-rationale"):
                bad = copy.deepcopy(review)
                entry = bad["mutation_equivalences"][0]
                if change == "candidate":
                    bad["candidate_sha256"] = "b" * 64
                elif change == "source":
                    entry["source_sha256"] = "b" * 64
                elif change == "inventory":
                    bad["inventories"]["host"]["sha256"] = "b" * 64
                elif change == "bool-count":
                    bad["inventories"]["host"]["count"] = True
                elif change == "unknown":
                    entry["mutant"]["name"] = "unlisted fault"
                elif change == "replacement":
                    entry["mutant"]["replacement"] = "true"
                elif change == "duplicate-id":
                    bad["mutation_equivalences"].append(copy.deepcopy(entry))
                elif change == "duplicate-fault":
                    duplicate = copy.deepcopy(entry)
                    duplicate["id"] = "FX-EQ-002"
                    bad["mutation_equivalences"].append(duplicate)
                elif change == "proof-hash":
                    entry["proof"]["sha256"] = "b" * 64
                elif change == "proof-section":
                    entry["proof"]["section"] = "FX-EQ-002"
                elif change == "no-proof":
                    del entry["proof"]
                elif change == "no-reviewer":
                    entry["proof"]["reviewer"] = ""
                else:
                    entry["rationale"] = ""
                with self.subTest(change=change), self.assertRaises(M.Incomplete):
                    M.admit_equivalence_review(bad, "a" * 64, inventories(), identity, root)
            source = root / "tools/windows-service-fixture/src/host.rs"
            source.write_text("changed source after review\n")
            with self.assertRaisesRegex(M.Incomplete, "source or full fixture candidate"):
                M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)

    def test_equivalence_requires_a_nonempty_canonical_proof_section_and_whole_file_hash(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            proof = root / "tasks/windows11-integration.md"
            for text in ("## unrelated section\nargument\n", "#### FX-EQ-001\n\n#### NEXT\nforeign argument\n"):
                proof.write_text(text)
                review["mutation_equivalences"][0]["proof"]["sha256"] = hashlib.sha256(proof.read_bytes()).hexdigest()
                with self.subTest(text=text), self.assertRaises(M.Incomplete):
                    M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)
            for path in ("../tasks/windows11-integration.md", "/tasks/windows11-integration.md", "artifacts/proof.md"):
                review["mutation_equivalences"][0]["proof"]["path"] = path
                with self.subTest(path=path), self.assertRaisesRegex(M.Incomplete, "canonical maintained task"):
                    M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)

    def test_equivalent_survivor_changes_only_adjusted_denominator_and_never_gets_kill_credit(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            review, identity = equivalence_fixture(root)
            entry = M.admit_equivalence_review(review, "a" * 64, inventories(), identity, root)[0]
            report = outcomes("MissedMutant")
            caught = descriptor(name="discriminated neighboring fault")
            report["outcomes"].append({"scenario": {"Mutant": caught}, "summary": "CaughtMutant",
                                       "phase_results": phases("CaughtMutant"), "log_path": "caught.log"})
            report.update(total_mutants=2, caught=1)
            expected = {"host guard": descriptor(), caught["name"]: caught}
            before = copy.deepcopy(report)
            result = M.C.mutation_counts(M.SCOPE, report, expected, "src/host.rs", native_exit_code=2,
                                         equivalences=[entry], defer_gate=True)
            self.assertTrue(result["gate_passed"])
            self.assertEqual((result["generated"], result["killed"], result["survived"], result["excluded"]), (2, 1, 1, 1))
            self.assertEqual((result["raw_denominator"], result["adjusted_denominator"]), (2, 1))
            self.assertEqual((result["raw_viable_killed_percent"], result["adjusted_viable_killed_percent"]), (50, 100))
            self.assertEqual(report, before)
            (root / "mutant.log").write_text(execution())
            cause = M.mutation_causes(root, outcomes("MissedMutant"), ["healthy_owner", "failed_cleanup"], [entry])
            self.assertEqual(cause["host guard"]["kind"], "reviewed-equivalent-survivor")
            self.assertEqual(cause["host guard"]["kill_credit"], 0)

    def test_equivalence_cannot_hide_unknown_changed_duplicate_nonmissed_faults_or_timeouts(self):
        entry = {"id": "FX-EQ-001", "mutant": descriptor(), "proof": {"reviewer": "reviewer"}}
        M.equivalent_misses(outcomes("MissedMutant"), [entry])
        for summary in ("CaughtMutant", "Unviable", "Timeout"):
            with self.subTest(summary=summary), self.assertRaises(M.Incomplete):
                M.equivalent_misses(outcomes(summary), [entry])
        for entries in ([entry, entry], [{**entry, "mutant": descriptor(name="unknown")}],
                        [{**entry, "mutant": {**descriptor(), "replacement": "true"}}]):
            with self.subTest(entries=entries), self.assertRaises(M.Incomplete):
                M.equivalent_misses(outcomes("MissedMutant"), entries)
        with self.assertRaises(M.Incomplete):
            M.C.mutation_counts(M.SCOPE, outcomes("Timeout"), inventories()["host"], "src/host.rs",
                                native_exit_code=3, equivalences=[entry], defer_gate=True)

    def test_only_actual_selected_semantic_assertion_is_a_kill_cause(self):
        text = ("test healthy_owner ... FAILED\n\n---- healthy_owner stdout ----\n"
                "thread 'healthy_owner' panicked at src/host_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n\nfailures:\n")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(text)
            checked = M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            self.assertEqual(checked["host guard"]["kind"], "behavioral-assertion")
            for bad in (text.replace("src/host_tests.rs", "foreign.rs"),
                        text.replace("assertion `left == right` failed", "test fixture construction failed"),
                        text + "No space left on device\n", text.replace("healthy_owner", "foreign_case")):
                (directory / "mutant.log").write_text(bad)
                with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            with self.assertRaises(M.Incomplete):
                M.owned_log(directory, "../mutant.log")

    def test_compiler_unviable_is_source_attributed_and_never_killed(self):
        diagnostic = {"reason": "compiler-message", "target": {"name": "nevrotick_windows_service_fixture"},
                      "message": {"level": "error", "message": "mismatched types",
                                  "spans": [{"is_primary": True, "file_name": "src/host.rs"}]}}
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(json.dumps(diagnostic) + "\n")
            causes = M.mutation_causes(directory, outcomes("Unviable"), ["healthy_owner"])
            self.assertEqual(causes["host guard"]["kill_credit"], 0)
            for file in ("foreign.rs", "../src/host.rs"):
                diagnostic["message"]["spans"][0]["file_name"] = file
                (directory / "mutant.log").write_text(json.dumps(diagnostic) + "\n")
                with self.subTest(file=file), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes("Unviable"), ["healthy_owner"])

    def test_nocapture_assertion_before_fatal_abort_is_distinct_from_abort_or_generic_panic(self):
        text = ("test healthy_owner ... \n"
                "thread 'healthy_owner' (12345) panicked at src/host_tests.rs:120:5:\n"
                "assertion `left == right` failed\n  left: 1\n right: 2\n"
                "process didn't exit successfully (signal: 6, SIGABRT: process abort signal)\n")
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "mutant.log").write_text(text)
            checked = M.mutation_causes(directory, outcomes(), ["healthy_owner"])
            self.assertEqual(checked["host guard"]["causes"], [{"test": "healthy_owner", "source": "src/host_tests.rs",
                              "line": 120, "kind": "unbuffered-semantic-assertion"}])
            for bad in ("process abort signal SIGABRT\n", text.replace("healthy_owner'", "foreign_thread'"),
                        text.replace("src/host_tests.rs", "foreign.rs"),
                        text.replace("assertion `left == right` failed", "test fixture construction failed"),
                        text.replace("assertion `left == right` failed", "arbitrary panic message"),
                        text + "Cannot allocate memory\n"):
                (directory / "mutant.log").write_text(bad)
                with self.subTest(bad=bad), self.assertRaises(M.Incomplete):
                    M.mutation_causes(directory, outcomes(), ["healthy_owner"])

    def test_native_complete_derivative_reuse_restoration_and_failure_retention(self):
        for scenario in ("healthy", "failed", "dirty", "interrupted"):
            with self.subTest(scenario=scenario), tempfile.TemporaryDirectory(prefix="native-derivative-control-") as directory:
                work = Path(directory)
                source = work / "source"
                owner = source / M.Q.TOOL / "src/host.rs"
                owner.parent.mkdir(parents=True)
                owner.write_bytes(b"original owner")
                dependency = source / "crates/nevrotick-engine/src/lib.rs"
                dependency.parent.mkdir(parents=True)
                dependency.write_bytes(b"real complete dependency")
                session = M.Session.__new__(M.Session)
                session.work, session.source, session.vendor = work, source, work
                session.identity = M.C.PRODUCT.inventory(source)
                session.record = {"commands": []}
                session.admitted = {"extensions": {"cargo-mutants": Path("/selected/cargo-mutants")}}
                def check():
                    self.assertEqual(M.C.PRODUCT.inventory(source), session.identity)
                session.check = check
                def observe(work_arg, command, admitted, vendor, derivative, **options):
                    self.assertEqual(derivative, work / "native-source")
                    self.assertEqual((derivative / "crates/nevrotick-engine/src/lib.rs").read_bytes(), b"real complete dependency")
                    self.assertEqual(options["native_subject"], "host")
                    self.assertEqual(options["cwd"], derivative / M.Q.TOOL)
                    target = derivative / M.Q.TOOL / "src/host.rs"
                    target.write_bytes(b"temporary mutation")
                    if scenario != "dirty": target.write_bytes(b"original owner")
                    if scenario == "interrupted": raise RuntimeError("interrupted native command")
                    return {"exit_code": 101 if scenario == "failed" else 0, "failure": None,
                            "output": "original native observation"}
                with patch.object(M.Q, "run_command", side_effect=observe) as execute:
                    if scenario == "healthy":
                        M.native_mutation_run(session, "host", 60)
                        M.native_mutation_run(session, "host", 60)
                        self.assertEqual(len(session.record["native_derivative"]["runs"]), 2)
                        self.assertEqual(execute.call_count, 2)
                        M.retire_native_derivative(session)
                        self.assertFalse((work / "native-source").exists())
                    else:
                        with self.assertRaises((M.Incomplete, RuntimeError)):
                            M.native_mutation_run(session, "host", 60)
                        self.assertTrue((work / "native-source").exists())
                        if scenario == "dirty":
                            self.assertEqual((work / "native-source" / M.Q.TOOL / "src/host.rs").read_bytes(), b"temporary mutation")
                            with self.assertRaises(M.Incomplete): M.retire_native_derivative(session)
                            self.assertIn("custody_error", session.record["native_derivative"]["runs"][0])
                        elif scenario == "failed":
                            self.assertEqual(session.record["commands"][0]["exit_code"], 101)
                        else:
                            self.assertIn("command_error", session.record["native_derivative"]["runs"][0])
                self.assertEqual(owner.read_bytes(), b"original owner")

    def test_fixed_commands_preserve_all_files_lints_baselines_and_resource_roles(self):
        admitted = {"extensions": {"cargo-mutants": Path("/selected/cargo-mutants")}}
        command = M.mutation_base_command(admitted, "host")
        self.assertEqual(command, ["/selected/cargo-mutants", "mutants", "--package",
            "nevrotick-windows-service-fixture", "--file", "src/host.rs", "--test-workspace", "false",
            "--cap-lints", "false", "--jobs", "4", "--jobserver", "true", "--jobserver-tasks", "4",
            "--cargo-arg=--message-format=json", "--cargo-arg=--frozen"])
        self.assertFalse(any(arg in command for arg in ("--exclude", "--skip", "--skip-call", "--in-place")))
        with self.assertRaises(M.Incomplete):
            M.mutation_base_command(admitted, "engine")

    def test_failed_native_command_is_retained_before_rejection_with_exact_resource_role(self):
        session = M.Session.__new__(M.Session)
        session.work, session.source, session.vendor = Path("/owned/work"), Path("/owned/work/source"), Path("/vendor")
        session.admitted, session.record = {"selected": "admitted"}, {"commands": []}
        session.check = lambda: None
        result = {"exit_code": 101, "failure": None, "output": "actual failed baseline"}
        with patch.object(M.Q, "run_command", return_value=result) as run, \
                patch.object(M.shutil, "disk_usage", return_value=type("Disk", (), {"free": 2**60})()), \
                self.assertRaises(M.Incomplete):
            session.run("mutation:host", ["literal-mutator", "mutants"], 117, mutation=True)
        run.assert_called_once_with(session.work, ["literal-mutator", "mutants"], session.admitted,
                                    session.vendor, session.source, timeout=117, cpu_seconds=117,
                                    mutation=True, cwd=session.source / "tools/windows-service-fixture")
        self.assertEqual(session.record["commands"][0]["output"], "actual failed baseline")

    def test_retention_preserves_profiles_binaries_outcomes_and_failed_runs(self):
        for path in ("work/target/llvm-cov-target/debug/fixture", "work/target/a.profraw",
                     "work/target/a.profdata", "work/mutation/host/mutants.out/outcomes.json",
                     "work/mutation/error/mutants.out/log/fault.log", "work/coverage.json"):
            with self.subTest(path=path):
                self.assertTrue(M.E.retained(path))
        self.assertFalse(M.E.retained("work/target/debug/derived-only"))
        with tempfile.TemporaryDirectory() as temporary:
            origin, ledger = Path(temporary) / "origin", Path(temporary) / "ledger"
            origin.mkdir(mode=0o700)
            ledger.mkdir(mode=0o700)
            target = origin / "work/target/llvm-cov-target/debug"
            target.mkdir(parents=True)
            (target / "fixture").write_bytes(b"LLVM instrumented executable")
            (origin / "failed.log").write_text("actual first failed attempt\n")
            with patch.object(M.E, "RECORDS", ledger), patch.object(M.E, "DISK_FLOOR", 0):
                M.E.export_tree(origin, ledger / "raw", ledger)
            self.assertEqual((ledger / "raw/failed.log").read_text(), "actual first failed attempt\n")
            self.assertEqual((ledger / "raw/work/target/llvm-cov-target/debug/fixture").read_bytes(),
                             b"LLVM instrumented executable")
            self.assertTrue(json.loads((ledger / "export-map.json").read_text())["complete"])

    def test_export_floor_default_and_four_gib_admission_and_copy_drop(self):
        floor = 4 * 1024 ** 3
        default = M.E.DISK_FLOOR
        class Disk:
            def __init__(self, free): self.free = free
        for selected, failed, dropping in ((None, False, False), (floor, False, False),
                                           (floor, True, False), (floor, True, True)):
            with self.subTest(selected=selected, failed=failed, dropping=dropping), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                origin, ledger = root / "origin", root / "ledger"
                origin.mkdir(); ledger.mkdir()
                (origin / "a.log").write_bytes(b"aaa")
                if dropping: (origin / "b.log").write_bytes(b"bbbb")
                chosen = default if selected is None else selected
                free = [Disk(chosen + 7), Disk(chosen + 3), Disk(chosen + 3)] if dropping else [
                    Disk(chosen + (2 if failed else 3)), Disk(chosen + 3)]
                kwargs = {} if selected is None else {"disk_floor": selected}
                with patch.object(M.E, "RECORDS", ledger), patch.object(M.E.shutil, "disk_usage", side_effect=free):
                    if failed:
                        with self.assertRaises(ValueError): M.E.export_tree(origin, ledger / "raw", ledger, **kwargs)
                    else:
                        M.E.export_tree(origin, ledger / "raw", ledger, **kwargs)
                plan = json.loads((ledger / "export-plan.json").read_text())
                self.assertEqual(plan["disk_floor_bytes"], chosen)
                if failed:
                    self.assertFalse((ledger / "export-map.json").exists())
                    if dropping:
                        self.assertEqual((ledger / "raw/a.log").read_bytes(), b"aaa")
                        self.assertFalse((ledger / "raw/b.log").exists())
                        self.assertEqual(len((ledger / "export-progress.jsonl").read_text().splitlines()), 1)
                    else:
                        self.assertFalse((ledger / "raw").exists())
                else:
                    self.assertEqual(json.loads((ledger / "export-map.json").read_text())["disk_floor_bytes"], chosen)
                self.assertEqual((origin / "a.log").read_bytes(), b"aaa")
        for invalid in (True, False, default - 1, float(floor), "4294967296"):
            with self.subTest(invalid=invalid), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                with self.assertRaises(ValueError): M.E.export_tree(root, root / "raw", root, disk_floor=invalid)
                self.assertFalse((root / "export-plan.json").exists())

    def test_metric_entrypoint_exports_with_service_four_gib_floor(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            records, checkout, cache = root / "records", root / "checkout", root / "cache"
            records.mkdir(); checkout.mkdir()
            args = type("Args", (), {"lane": "coverage", "label": "owned-export",
                "equivalence_review": None, "direct_inventory": None})()
            with patch.object(M.Q, "CACHE", cache), patch.object(M.Q, "RECORDS", records), \
                    patch.object(M.Q, "ROOT", checkout), patch.object(M.Q, "no_ambient_flags"), \
                    patch.object(M.Q, "disable_core_dumps", return_value={}), \
                    patch.object(M.shutil, "disk_usage", return_value=type("Disk", (), {"free": 2**60})()), \
                    patch.object(M, "Session"), patch.object(M, "execute"), \
                    patch.object(M.E, "export_tree") as export:
                self.assertEqual(M.run(args), "passed")
            export.assert_called_once_with(cache / args.label, records / "owned-export-evidence/raw",
                                           records / "owned-export-evidence", disk_floor=M.Q.DISK_FLOOR)
            self.assertEqual(M.Q.DISK_FLOOR, 4 * 1024 ** 3)


if __name__ == "__main__":
    unittest.main()
