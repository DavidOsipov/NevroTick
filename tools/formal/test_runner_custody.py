"""Formal custody regressions using private sentinels and a mocked native seam.

No compiler, solver, namespace or installed analyzer executes. Mathematical
qualification is mocked deliberately: these controls assess evidence custody.
"""
from __future__ import annotations

import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent

def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded

V = load("custody_formal_runner_test", HERE / "verify.py")
REAL = load("custody_real_snapshot_test", ROOT / "scripts/check-analysis.py")

class CustodyTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-formal-custody-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.checkout = self.base / "checkout"
        (self.checkout / "tools/formal").mkdir(parents=True)
        (self.checkout / "tools/formal/verify.py").write_bytes(b"frozen fixture runner bytes")
        (self.checkout / "fixture.rs").write_bytes(b"frozen fixture source bytes")
        self.work = self.base / "evidence"
        self.installation = self.base / "installation"
        self.stable = self.base / "stable"
        self.installation.mkdir()
        self.stable.mkdir()
        self.calls = []
        self.action = None
        self.final_action = None
        self.admission_action = None
        self.admission_calls = 0
        self.jobs = [
            {"subject":"rounding","case":"healthy","harness":"phase_full_domain",
             "intended_failure":None,"restored_healthy":False},
            {"subject":"rounding","case":"healthy","harness":"round_trip_full_domain",
             "intended_failure":None,"restored_healthy":False},
        ]
        def native(root, work, command, **options):
            self.calls.append(options)
            REAL.RUNTIME._sealed_inputs(work, options["sealed"])
            folder = options["cwd"].parent
            (folder / "target").mkdir()
            (folder / "target/disposable.bin").write_bytes(b"disposable compile output")
            if self.action:
                self.action(len(self.calls), work, folder)
            (folder / "kani.json").write_text("{}\n")
            return {"exit_code":0,"failure":None,"output":"mock native completion"}
        self.check = SimpleNamespace(
            read_once=REAL.read_once, decode=REAL.decode, input_snapshot=REAL.input_snapshot,
            PROFILES=REAL.PROFILES,
            RUNTIME=SimpleNamespace(run=native, _owned_work=REAL.RUNTIME._owned_work))

    def admission(self, check, output, **options):
        self.admission_calls += 1
        if self.admission_action:
            self.admission_action(self.admission_calls)
        if output.name == "final-admission.json" and self.final_action:
            self.final_action()
        record={"seconds":0.0}
        V.write_json(output, record)
        admitted={"base":self.installation,"stable":self.stable,
            "readonly_roots":(self.installation,self.stable),
            "policy":{"tools":{"kani":{"entrypoint":V.KANI_ENTRYPOINT,"toolchain":V.KANI_COMPILER}}}}
        return admitted, record

    def prepare(self, source, inputs, case):
        entry=inputs / "proof.rs"
        entry.write_bytes(b"// mock proof input, never compiled\n")
        return {"entrypoint":entry}

    def execute(self):
        with patch.object(V,"ROOT",self.checkout), \
             patch.object(V,"module",return_value=self.check), \
             patch.object(V,"input_names",return_value=["fixture.rs","tools/formal/verify.py"]), \
             patch.object(V,"jobs",return_value=self.jobs), \
             patch.object(V,"prepare_rounding",side_effect=self.prepare), \
             patch.object(V,"admission",side_effect=self.admission), \
             patch.object(V,"qualify",return_value={"solver_seconds":0.0,"checks":1}), \
             patch("sys.stdout",new=io.StringIO()):
            result=V.main(["--output-dir",str(self.work),"--subject","rounding"])
        return result,json.loads((self.work / "summary.json").read_text())

    def first(self):
        return self.work / "jobs/01-rounding-healthy-phase_full_domain"

    def test_plain_healthy_control_completes_both_mocked_commands(self):
        result, summary = self.execute()
        self.assertEqual(result, 0, summary)
        self.assertEqual(summary["status"], "passed")
        self.assertTrue(all(job["qualified"] for job in summary["jobs"]))
        self.assertEqual(len(self.calls), 2)

    def test_admission_cannot_change_prior_receipt_before_a_later_command(self):
        def tamper(count):
            if count == 2:
                (self.first() / "kani.json").write_text('{"tampered":true}\n')
        self.admission_action = tamper
        result, summary = self.execute()
        self.assertEqual(result, 1, summary)
        self.assertEqual(len(self.calls), 1)

    def test_healthy_control_protects_source_prior_islands_and_receipts(self):
        result,summary=self.execute()
        self.assertEqual(result,0,summary)
        self.assertEqual(summary["status"],"passed")
        self.assertEqual(len(self.calls),2)
        second=self.calls[1]["sealed"]
        for path in (self.work/"source",self.work/"source-manifest.json",self.first()/"inputs",
                     self.first()/"execution.json",self.first()/"kani.json",self.work/"summary.json"):
            self.assertIn(path,second)
        self.assertNotIn(self.first()/"target",second)
        self.assertTrue(summary["all_proof_inputs_and_required_receipts_unchanged"])

    def test_later_job_cannot_change_previous_generated_input(self):
        def tamper(count,work,folder):
            if count==2:
                path=self.first()/"inputs/proof.rs"
                path.write_bytes(path.read_bytes()+b"// changed by later command\n")
        self.action=tamper
        result,summary=self.execute()
        self.assertEqual(result,1,summary)
        self.assertEqual(summary["status"],"incomplete")

    def test_later_job_cannot_change_previous_raw_receipt(self):
        def tamper(count,work,folder):
            if count==2:
                (self.first()/"kani.json").write_text('{"forged":true}\n')
        self.action=tamper
        result,summary=self.execute()
        self.assertEqual(result,1,summary)
        self.assertEqual(summary["status"],"incomplete")
        self.assertEqual(len(summary["jobs"]), 2)
        self.assertFalse(summary["jobs"][-1]["qualified"])
        self.assertNotIn("all_proof_inputs_and_required_receipts_unchanged", summary)

    def test_extra_frozen_source_file_is_rejected(self):
        def tamper(count,work,folder):
            if count==1:
                (work/"source/unregistered.py").write_text("# added\n")
        self.action=tamper
        result,summary=self.execute()
        self.assertEqual(result,1,summary)

    def test_final_admission_cannot_hide_prior_input_drift(self):
        self.final_action=lambda:(self.first()/"inputs/proof.rs").write_text("changed at final admission\n")
        result,summary=self.execute()
        self.assertEqual(result,1,summary)
        self.assertIn("final_error",summary)

    def test_disposable_old_target_changes_do_not_change_evidence_scope(self):
        def tamper(count,work,folder):
            if count==2:
                (self.first()/"target/disposable.bin").write_bytes(b"allowed disposable change")
        self.action=tamper
        result,summary=self.execute()
        self.assertEqual(result,0,summary)

    def test_artifact_writer_preserves_external_symlink_and_hardlink_targets(self):
        healthy=self.base/"healthy.json"
        V.write_json(healthy,{"healthy":True})
        self.assertEqual(json.loads(healthy.read_text()),{"healthy":True})
        for kind in ("symlink","hardlink"):
            sentinel=self.base/(kind+"-outside.txt")
            sentinel.write_bytes(b"PRESERVE")
            alias=self.base/(kind+"-receipt.json")
            if kind=="symlink":alias.symlink_to(sentinel)
            else:os.link(sentinel,alias)
            rejected=False
            try:V.write_json(alias,{"must_not_escape":True})
            except (V.Incomplete,OSError):rejected=True
            self.assertEqual(sentinel.read_bytes(),b"PRESERVE")
            self.assertTrue(rejected)

    def test_artifact_parent_symlink_is_rejected_without_touching_outside_data(self):
        outside=self.base/"outside-directory";outside.mkdir()
        sentinel=outside/"receipt.json";sentinel.write_bytes(b"PRESERVE")
        alias=self.base/"parent-alias";alias.symlink_to(outside,target_is_directory=True)
        rejected=False
        try:V.write_json(alias/"receipt.json",{"must_not_escape":True})
        except (V.Incomplete,OSError):rejected=True
        self.assertEqual(sentinel.read_bytes(),b"PRESERVE")
        self.assertTrue(rejected)

    def test_native_created_receipt_alias_never_truncates_external_sentinel(self):
        sentinel=self.base/"native-outside.txt";sentinel.write_bytes(b"PRESERVE")
        def tamper(count,work,folder):
            if count==2:
                path=folder/"execution.json";path.unlink();path.symlink_to(sentinel)
        self.action=tamper
        result,summary=self.execute()
        self.assertEqual(sentinel.read_bytes(),b"PRESERVE")
        self.assertEqual(result,1,summary)
        self.assertEqual(len(summary["jobs"]), 2)
        self.assertFalse(summary["jobs"][-1]["qualified"])
        self.assertNotIn("all_proof_inputs_and_required_receipts_unchanged", summary)

if __name__=="__main__":unittest.main()
