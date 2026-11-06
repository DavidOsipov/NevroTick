"""Independent contract-result and callee-gate controls; execute no verifier."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import Mock, patch


HERE = Path(__file__).resolve().parent


def load(name, filename):
    specification = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


VERIFY = load("fraction_contracts_controller_tests", "verify_fraction_contracts.py")
FIXTURE = load("fraction_contracts_ordinary_json_fixture", "test_verify.py")
ENTRY = FIXTURE.ENTRY
SELECTED = "/work/inputs/fraction_contracts_selected.rs"
IDENTITY = "a" * 64
BINDING = "b" * 64


def checking_job(identifier="constructor"):
    target = {"constructor": "Fraction::new", "complement": "Fraction::complement",
              "mechanism_callee": "primitive"}[identifier]
    clause = {
        "constructor": "|result: &Result<Self, FixtureEnclosureError>| constructor_post(numerator, shift, result)",
        "complement": "|result: &Result<Self, FixtureEnclosureError>| complement_post(self, result)",
        "mechanism_callee": "|result: &u8| *result > value",
    }[identifier]
    return {
        "id": identifier, "subject": "contract_check", "case": "healthy",
        "harness": "fraction_contracts::check_" + identifier,
        "expected_kind": 'ProofForContract { target_fn: "' + target + '" }',
        "verified_stubs": ["Fraction::new"] if identifier == "complement" else [],
        "dependencies": ["constructor"] if identifier == "complement" else [],
        "expected_assertions": ["full raw contract classification"],
        "expected_covers": ["accepted raw witness", "rejected raw witness"],
        "required_contract_assertions": [clause],
        "required_precondition_assertions": [], "intended_failure": None,
    }


def caller_job():
    return {
        "id": "addition_healthy", "subject": "addition", "case": "healthy",
        "harness": "fraction_contracts::addition_full_domain", "expected_kind": "Proof",
        "verified_stubs": ["Fraction::new", "Fraction::complement"],
        "dependencies": ["constructor", "complement"],
        "expected_assertions": ["addition required success", "addition exact carry", "addition exact residual"],
        "expected_covers": ["zero sum", "exact unit sum", "fractional sum"],
        "required_contract_assertions": [],
        "required_precondition_assertions": ["checked_fraction(self)"], "intended_failure": None,
    }


def result_metadata(report, job):
    """Checking has an opaque native symbol; an ordinary caller has null metadata."""
    metadata = report["harness_metadata"][0]
    metadata["attributes"]["kind"] = job["expected_kind"]
    metadata["contract"] = {
        "contracted_function_name": "_RNv_opaque_native_contract_symbol"
        if job["expected_kind"].startswith("ProofForContract {") else None,
        "recursion_tracker": None,
    }
    return report


def native_output(job, *, success=True):
    output = "Checking harness " + job["harness"] + "...\n"
    output += "".join("- Verified stub: proof::fraction_contracts::" + name + "\n"
                      for name in job["verified_stubs"])
    return {"exit_code": 0 if success else 1, "failure": None, "output": output}


def healthy(job=None):
    job = copy.deepcopy(job or checking_job())
    checks = [{"category": "assertion", "description": label, "status": "Success",
               "location": {"file": ENTRY}} for label in job["expected_assertions"]]
    checks += [{"category": "cover", "description": label, "status": "Satisfied",
                "location": {"file": ENTRY}} for label in job["expected_covers"]]
    for field in ("required_contract_assertions", "required_precondition_assertions"):
        checks += [{"category": "assertion", "description": clause, "status": "Success",
                    "location": {"file": SELECTED}} for clause in job[field]]
    report = result_metadata(FIXTURE.result(job, checks, True), job)
    return job, report, native_output(job)


def negative(kind):
    job = checking_job("mechanism_callee") if kind == "postcondition" else caller_job()
    job["id"] = "mechanism_bad_callee" if kind == "postcondition" else "mechanism_precondition"
    job["case"] = "bad_" + kind
    job["intended_failure"] = ("|result: &u8| *result > value" if kind == "postcondition"
                               else "value < u8::MAX")
    checks = [{"category": "assertion", "description": job["intended_failure"],
               "status": "Failure", "location": {"file": SELECTED}}]
    report = result_metadata(FIXTURE.result(job, checks, False), job)
    return job, report, native_output(job, success=False)


class ResultSchemaControls(unittest.TestCase):
    def qualify(self, values):
        job, report, native = values
        return VERIFY.qualify_result(report, native, job, ENTRY, allowed_files=(ENTRY, SELECTED))

    def test_checking_and_caller_have_nonempty_healthy_neighbor_controls(self):
        for job in (checking_job(), checking_job("complement"), caller_job()):
            with self.subTest(harness=job["harness"]):
                observed = self.qualify(healthy(job))
                self.assertGreater(observed["checks"], 0)

    def test_checking_requires_its_exact_kind_and_nonempty_opaque_symbol(self):
        for kind in ("Proof", "ProofForContract", None,
                     'ProofForContract { target_fn: "unrelated" }'):
            values = healthy()
            values[1]["harness_metadata"][0]["attributes"]["kind"] = kind
            with self.subTest(kind=kind), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)
        for symbol in (None, "", " ", 7, [], {}):
            values = healthy()
            values[1]["harness_metadata"][0]["contract"]["contracted_function_name"] = symbol
            with self.subTest(symbol=symbol), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)
        self.qualify(healthy())

    def test_ordinary_caller_requires_null_contract_symbol_and_no_recursion(self):
        for symbol in ("opaque", "", 1):
            values = healthy(caller_job())
            values[1]["harness_metadata"][0]["contract"]["contracted_function_name"] = symbol
            with self.subTest(symbol=symbol), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)
        for job in (checking_job(), caller_job()):
            values = healthy(job)
            values[1]["harness_metadata"][0]["contract"]["recursion_tracker"] = "recursive"
            with self.subTest(job=job["id"]), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)

    def test_contract_metadata_is_closed_and_malformed_attributes_are_incomplete(self):
        for malformed in (None, [], "opaque", {}, {"contracted_function_name": "opaque"},
                          {"contracted_function_name": "opaque", "recursion_tracker": None, "extra": True}):
            values = healthy()
            values[1]["harness_metadata"][0]["contract"] = malformed
            with self.subTest(contract=malformed), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)
        for malformed in (None, [], "opaque", 1):
            values = healthy()
            values[1]["harness_metadata"][0]["attributes"] = malformed
            with self.subTest(attributes=malformed), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)

    def test_resolved_stdout_stubs_are_an_exact_set_with_no_unverified_replacements(self):
        original = healthy(caller_job())
        outputs = [
            original[2]["output"].replace("- Verified stub: proof::fraction_contracts::Fraction::new\n", ""),
            original[2]["output"] + "- Verified stub: proof::fraction_contracts::Fraction::new\n",
            original[2]["output"].replace("Fraction::complement", "Fraction::compare"),
            original[2]["output"].replace("proof::fraction_contracts::Fraction::new", "other::Fraction::new"),
            original[2]["output"] + "- Stub: arbitrary_replacement\n",
            original[2]["output"] + "Checking harness unrelated...\n",
            original[2]["output"] + "Thread 1: parallel verification\n",
        ]
        for output in outputs:
            values = copy.deepcopy(original)
            values[2]["output"] = output
            with self.subTest(output=output), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)
        self.qualify(original)

    def test_required_checker_postcondition_and_caller_precondition_must_succeed(self):
        for job in (checking_job(), caller_job()):
            original = healthy(job)
            original_checks = original[1]["verification_results"]["results"][0]["checks"]
            contract_index = len(original_checks) - 1
            for alteration in ("missing", "Unreachable", "Failure", "different clause", "wrong category"):
                values = copy.deepcopy(original)
                checks = values[1]["verification_results"]["results"][0]["checks"]
                if alteration == "missing":
                    del checks[contract_index]
                elif alteration == "different clause":
                    checks[contract_index]["description"] = "unrelated assertion"
                elif alteration == "wrong category":
                    checks[contract_index]["category"] = "sanity"
                else:
                    checks[contract_index]["status"] = alteration
                FIXTURE.recount(values[1])
                with self.subTest(job=job["id"], alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)
            self.qualify(original)

    def test_tautological_or_negated_clause_containers_cannot_replace_the_required_contract(self):
        for job in (checking_job(), caller_job()):
            field = "required_contract_assertions" if job["id"] == "constructor" else "required_precondition_assertions"
            clause = job[field][0]
            for weakened in ("true || (" + clause + ")", "!(" + clause + ")", clause + " || true"):
                values = healthy(job)
                values[1]["verification_results"]["results"][0]["checks"][-1]["description"] = weakened
                with self.subTest(job=job["id"], weakened=weakened), self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)
            quoted = healthy(job)
            quoted[1]["verification_results"]["results"][0]["checks"][-1]["description"] = '"' + clause + '"'
            self.qualify(quoted)

    def test_postcondition_and_precondition_controls_require_only_their_intended_failure(self):
        for kind in ("postcondition", "precondition"):
            original = negative(kind)
            self.assertEqual(len(self.qualify(original)["failed_checks"]), 1)
            for alteration in ("unrelated assertion", "wrong file", "timeout", "zero exit"):
                values = copy.deepcopy(original)
                check = values[1]["verification_results"]["results"][0]["checks"][0]
                if alteration == "unrelated assertion":
                    check["description"] = "unrelated assertion"
                elif alteration == "wrong file":
                    check["location"]["file"] = "/work/unrelated.rs"
                elif alteration == "timeout":
                    values[2]["failure"] = "wall timeout (180s)"
                else:
                    values[2]["exit_code"] = 0
                with self.subTest(kind=kind, alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)

    def test_empty_partial_unresolved_and_accounting_mismatches_remain_incomplete(self):
        for alteration in ("empty", "partial", "counter", "unknown status", "missing cover", "unreachable math"):
            values = healthy()
            report = values[1]
            checks = report["verification_results"]["results"][0]["checks"]
            if alteration == "empty":
                checks.clear()
                FIXTURE.recount(report)
            elif alteration == "partial":
                report["verification_results"]["summary"]["status"] = "incomplete"
            elif alteration == "counter":
                report["property_details"][0]["property_details"]["passed"] += 1
            elif alteration == "unknown status":
                checks[0]["status"] = "Unknown"
            elif alteration == "missing cover":
                del checks[1]
                FIXTURE.recount(report)
            else:
                checks[0]["status"] = "Unreachable"
                FIXTURE.recount(report)
            with self.subTest(alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)

    def test_ordinary_role_stays_closed_to_contract_kinds_and_new_flags(self):
        ordinary = FIXTURE.healthy()
        FIXTURE.VERIFY.qualify(ordinary[1], ordinary[2], ordinary[0], ENTRY)
        ordinary[1]["harness_metadata"][0]["attributes"]["kind"] = checking_job()["expected_kind"]
        with self.assertRaises(FIXTURE.VERIFY.Incomplete):
            FIXTURE.VERIFY.qualify(ordinary[1], ordinary[2], ordinary[0], ENTRY)
        flags = list(VERIFY.BASE.KANI_BASELINE_FLAGS)
        check = SimpleNamespace(PROFILES={"kani_flags": flags})
        self.assertEqual(VERIFY.BASE.solver_flags(check), flags)
        check.PROFILES["kani_flags"] = [*flags, "-Z", "function-contracts", "-Z", "stubbing"]
        with self.assertRaises(VERIFY.Incomplete):
            VERIFY.BASE.solver_flags(check)


class CalleeGateControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-contract-callee-gate-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.check = SimpleNamespace(input_snapshot=self.snapshot, read_once=self.read_once, decode=json.loads,
                                     RUNTIME=SimpleNamespace(run=Mock(), _owned_work=Mock()))

    @staticmethod
    def read_once(path, **kwargs):
        return Path(path).read_bytes()

    @staticmethod
    def snapshot(work, roots):
        result = {}
        for root in roots:
            root = Path(root)
            for path in ([root, *sorted(root.rglob("*"))] if root.is_dir() else [root]):
                key = path.relative_to(work).as_posix()
                result[key] = {"kind": "directory"} if path.is_dir() else {
                    "kind": "file", "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
        return result

    def record(self, identifier="constructor"):
        job, report, native = healthy(checking_job(identifier))
        folder = self.work / (identifier + "-callee")
        inputs = folder / "inputs"
        inputs.mkdir(parents=True)
        (inputs / "proof.rs").write_text("// independent synthetic checking input\n")
        record = {"job": job, "output": str(folder), "native": native,
                  "expected_outcome_qualified": True, "checks": 4, "input_unchanged": True,
                  "tool_identity": IDENTITY, "preparation": {"callee_bindings": {identifier: BINDING}}}
        for name in ("input-before.json", "input-after.json", "admission.json"):
            (folder / name).write_text("{}\n")
        (folder / "kani.json").write_text(json.dumps(report))
        (folder / "execution.json").write_text(json.dumps(record))
        return record

    def certificate(self, identifier="constructor"):
        record = self.record(identifier)
        return VERIFY.issue_certificate(self.check, self.work, record), record

    def require(self, certificate, record=None):
        job = caller_job()
        job["dependencies"] = ["constructor"]
        prepared = {"callee_bindings": {"constructor": BINDING}}
        return VERIFY.require_callees(job, prepared, {"constructor": certificate}, IDENTITY, self.check, self.work)

    def rewrite_retained(self, certificate, record):
        Path(certificate["execution_receipt"]).write_text(json.dumps(record))
        certificate["receipt_snapshot"] = self.snapshot(self.work, [Path(p) for p in certificate["receipt_roots"]])

    def test_only_healthy_nonempty_checking_record_can_issue_custody_bound_certificate(self):
        certificate, record = self.certificate()
        self.assertTrue(certificate["qualified"])
        self.assertEqual(certificate["job_id"], "constructor")
        self.assertEqual(certificate["dependency_binding"], BINDING)
        self.assertEqual(certificate["tool_identity"], IDENTITY)
        self.require(certificate)
        self.check.RUNTIME.run.assert_not_called()

    def test_expected_negative_control_never_issues_a_callee_certificate(self):
        record = self.record()
        for alteration in ("negative case", "negative job", "ordinary proof", "unqualified", "failed exit", "empty checks"):
            changed = copy.deepcopy(record)
            if alteration == "negative case":
                changed["job"]["case"] = "bad_postcondition"
            elif alteration == "negative job":
                changed["job"]["id"] = "mechanism_bad_callee"
            elif alteration == "ordinary proof":
                changed["job"]["expected_kind"] = "Proof"
            elif alteration == "unqualified":
                changed["expected_outcome_qualified"] = False
            elif alteration == "failed exit":
                changed["native"]["exit_code"] = 1
            else:
                changed["checks"] = 0
            with self.subTest(alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                VERIFY.issue_certificate(self.check, self.work, changed)

    def test_certificate_issue_rejects_native_failure_or_changed_inputs_despite_claimed_qualification(self):
        record = self.record()
        for alteration in ("native failure", "changed inputs", "missing native failure field", "boolean checks", "negative checks"):
            changed = copy.deepcopy(record)
            if alteration == "native failure":
                changed["native"]["failure"] = "wall timeout (180s)"
            elif alteration == "changed inputs":
                changed["input_unchanged"] = False
            elif alteration == "missing native failure field":
                del changed["native"]["failure"]
            elif alteration == "boolean checks":
                changed["checks"] = True
            else:
                changed["checks"] = -1
            with self.subTest(alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                VERIFY.issue_certificate(self.check, self.work, changed)

    def test_failed_missing_stale_or_incomplete_callee_evidence_is_refused(self):
        certificate, record = self.certificate()
        for alteration in ("missing", "unqualified", "stale tool", "stale binding", "stale receipt"):
            changed = copy.deepcopy(certificate)
            if alteration == "missing":
                changed = None
            elif alteration == "unqualified":
                changed["qualified"] = False
            elif alteration == "stale tool":
                changed["tool_identity"] = "different tool bytes"
            elif alteration == "stale binding":
                changed["dependency_binding"] = "different body/spec/type/generator bytes"
            else:
                changed["receipt_snapshot"] = {}
            with self.subTest(alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                self.require(changed)
        for alteration in ("unqualified", "wrong id", "negative case", "ordinary kind", "failed exit",
                           "native failure", "missing checks", "changed inputs"):
            changed = copy.deepcopy(record)
            if alteration == "unqualified": changed["expected_outcome_qualified"] = False
            elif alteration == "wrong id": changed["job"]["id"] = "unrelated"
            elif alteration == "negative case": changed["job"]["case"] = "bad_postcondition"
            elif alteration == "ordinary kind": changed["job"]["expected_kind"] = "Proof"
            elif alteration == "failed exit": changed["native"]["exit_code"] = 1
            elif alteration == "native failure": changed["native"]["failure"] = "timeout"
            elif alteration == "missing checks": changed["checks"] = 0
            else: changed["input_unchanged"] = False
            self.rewrite_retained(certificate, changed)
            with self.subTest(alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                self.require(certificate)
        self.rewrite_retained(certificate, record)
        self.require(certificate)

    def test_raw_kani_receipt_or_callee_source_drift_invalidates_an_existing_certificate(self):
        certificate, record = self.certificate()
        folder = Path(record["output"])
        for path in (folder / "kani.json", folder / "inputs/proof.rs"):
            original = path.read_bytes()
            path.write_bytes(original + b"\nchanged\n")
            with self.subTest(path=path), self.assertRaises(VERIFY.Incomplete):
                self.require(certificate)
            path.write_bytes(original)
        self.require(certificate)

    def test_dependent_caller_is_never_dispatched_without_valid_callee_receipts(self):
        certificate, record = self.certificate()
        bad_record = copy.deepcopy(record)
        bad_record["native"]["exit_code"] = 1
        bad_certificate = copy.deepcopy(certificate)
        self.rewrite_retained(bad_certificate, bad_record)
        examples = ({}, {"constructor": bad_certificate})
        for index, certificates in enumerate(examples):
            job = caller_job()
            job["id"] = "blocked_caller_" + str(index)
            job["dependencies"] = ["constructor"]
            source = self.work / "source"
            source.mkdir(exist_ok=True)
            def prepare(_source, inputs, _job):
                (inputs / "proof.rs").write_text("// checking harness exists; this is not a callee proof\n")
                return {"entrypoint": inputs / "proof.rs", "callee_bindings": {"constructor": BINDING}}
            admitted = {"base": self.work / "store", "policy": {"tools": {"kani": {
                "entrypoint": "kani/payload/kani-0.68.0/bin/kani-driver",
                "toolchain": "kani/rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"}}},
                "readonly_roots": ()}
            policy = {"flags": list(VERIFY.BASE.KANI_BASELINE_FLAGS), "rustflags": VERIFY.BASE.RUSTFLAGS,
                      "command_wall_seconds": 180, "command_cpu_seconds": 180,
                      "build_jobs": 2, "address_space_bytes": 8 * 1024**3, "process_limit": 128}
            with patch.object(VERIFY, "remaining_seconds", return_value=100), \
                 patch.object(VERIFY, "require_sources"), \
                 patch.object(VERIFY, "admission", return_value=(admitted, {"seconds": 0})), \
                 patch.object(VERIFY, "tool_identity", return_value=IDENTITY), \
                 patch.object(VERIFY.BASE, "audit_retained"), \
                 patch.object(VERIFY.BASE, "retain_artifacts"):
                result = VERIFY.execute_job(self.check, self.work, source, {}, SimpleNamespace(prepare=prepare),
                                            policy, job, index + 1, [], certificates)
            self.assertFalse(result["expected_outcome_qualified"])
            self.assertIn("callee", result["qualification_error"].lower())
            self.check.RUNTIME.run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
