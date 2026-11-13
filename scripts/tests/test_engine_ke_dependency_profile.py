"""Closed EP1 graph-profile controls; no Cargo or native execution."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

BASE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("ep1_dependency_fixtures", BASE / "test_slint_dependency_profile.py")
FIXTURE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(FIXTURE)
VERIFY, PROFILE = FIXTURE.VERIFY, FIXTURE.PROFILE


class EngineKeProfileControls(unittest.TestCase):
    def test_explicit_profile_selects_both_targets_and_exact_feature(self):
        rows = VERIFY.commands("dependencies", schema_version=3, profile="engine-ke-association")
        targets = set()
        for label, command in rows:
            if "cargo" in command and ("metadata" in command or "tree" in command):
                selected = ("metadata" in command or "--workspace" in command or
                            "--package" in command and command[command.index("--package") + 1] == "nevrotick-engine")
                if selected:
                    self.assertIn("--features", command)
                    self.assertEqual(command[command.index("--features") + 1], "nevrotick-engine/ke-association-prototype")
                else:
                    self.assertNotIn("--features", command)
                self.assertNotIn("--all-features", command)
                flag = "--filter-platform" if "metadata" in command else "--target"
                targets.add(command[command.index(flag) + 1])
        self.assertEqual(targets, {FIXTURE.HOST, FIXTURE.WINDOWS})
        self.assertEqual(VERIFY.commands("dependencies", schema_version=3),
                         VERIFY.commands("dependencies", schema_version=3, profile="default"))

    def test_foreign_profile_unknown_lane_and_old_schema_refuse(self):
        for lane, profile in (("dependencies", "foreign"), ("portable", "foreign"),
                              ("windows", "foreign"), ("foreign", "engine-ke-association")):
            with self.subTest(lane=lane, profile=profile), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.commands(lane, schema_version=3, profile=profile)
        for lane in ("dependencies", "portable", "windows"):
            with self.subTest(lane=lane), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.commands(lane, schema_version=2, profile="engine-ke-association")

    def test_default_commands_preserve_pre_extension_bytes(self):
        # Frozen command-list identities before extending EP1 beyond dependencies.
        expected = {
            (2, (), "dependencies"): "d97186f0130fa1c3df2db5d91f23863b244ce1612b888b481f98171c3cb19520",
            (2, (), "portable"): "6bae25fcc559a08fafdc923002d89c8ed3190342b80d79f1fd65db6363b4efaf",
            (2, (), "windows"): "eefebb2d44d144667cbb1699c09cdca8faf2f9731bb9d28b6a4410dd86c0d554",
            (3, (), "dependencies"): "c0688575d7a630dc5ddaa69271b3982125a2bc8cb250f7601bc87f5e626eb7e6",
            (3, (), "portable"): "0e480699228856ea37bd1ef0a905abf4a7128c989a449bd1cf836510c5761840",
            (3, (), "windows"): "0641c9021308191c276e53bdb78907f7143e8b28e07a6ef3b23e624b18aa0d66",
            (3, ("spin_on", "accesskit_windows", "accesskit_consumer"), "dependencies"):
                "dd475f4ace066cb4989af9697c631c34fdc9504053a4e043a9887166f055b982",
            (3, ("spin_on", "accesskit_windows", "accesskit_consumer"), "portable"):
                "d9daea5303ece7a38b027db822d6224a5f3bc62cf878e3717f29d2a9286ba6ce",
            (3, ("spin_on", "accesskit_windows", "accesskit_consumer"), "windows"):
                "a3c3252e7da60d3ed1a4a7d2217cd4ce11d6910cc7eab43fc966a38ae3818a8b",
        }
        for (schema, patches, lane), digest in expected.items():
            rows = VERIFY.commands(lane, schema_version=schema, patched_packages=patches)
            with self.subTest(schema=schema, patches=patches, lane=lane):
                # Restored pre-upgrade worker settings match these exact
                # historical command bytes; no normalization is admitted.
                actual = hashlib.sha256(json.dumps(rows, separators=(",", ":")).encode()).hexdigest()
                self.assertEqual(actual, digest)
                self.assertEqual(rows, VERIFY.commands(lane, schema_version=schema,
                    patched_packages=patches, profile="default"))

    def test_feature_reaches_only_engine_and_workspace_ordinary_commands(self):
        selected = {
            "portable": {"compiler", "clippy", "host-engine-build", "rustdoc",
                         "product-tests:nevrotick-engine", "doctests:nevrotick-engine",
                         "optimized-tests:nevrotick-engine"},
            "windows": {"windows-compiler", "windows-clippy", "windows-engine-link"},
        }
        for lane, required in selected.items():
            default = dict(VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on",)))
            prototype = dict(VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on",),
                                            profile="engine-ke-association"))
            self.assertEqual(set(default), set(prototype))
            actual = set()
            for label, command in prototype.items():
                if label.startswith("dependency-"):
                    if "--package" not in command or command[command.index("--package") + 1] == "nevrotick-engine":
                        self.assertEqual(command[-2:], ["--features", "nevrotick-engine/ke-association-prototype"])
                    else:
                        self.assertEqual(command, default[label])
                    continue
                if label in required:
                    actual.add(label)
                    self.assertEqual(command.count("--features"), 1)
                    index = command.index("--features")
                    self.assertEqual(command[index + 1], "nevrotick-engine/ke-association-prototype")
                    if "--" in command:
                        self.assertLess(index, command.index("--"))
                    expected = default[label].copy()
                    job_flag = "--build-jobs" if "--build-jobs" in expected else "--jobs"
                    expected[expected.index(job_flag) + 1] = "6"
                    self.assertEqual(command[:index] + command[index + 2:], expected)
                else:
                    self.assertEqual(command, default[label])
                self.assertNotIn("--all-features", command)
            self.assertEqual(actual, required)

    def test_feature_build_concurrency_is_six_without_other_argument_changes(self):
        """Bound concurrent LLVM jobs after the actual 128-task setup failure."""
        patches = ("spin_on", "accesskit_windows", "accesskit_consumer",
                   "rustls-cng-crypto", "rustls-platform-verifier")
        for lane in ("portable", "windows"):
            ordinary = dict(VERIFY.commands(lane, schema_version=3, patched_packages=patches))
            for label, command in VERIFY.commands(lane, schema_version=3,
                                                 patched_packages=patches, profile="engine-ke-association"):
                if label.startswith("dependency-"):
                    continue
                if "--features" not in command:
                    self.assertEqual(command, ordinary[label])
                    continue
                with self.subTest(lane=lane, label=label):
                    job_flag = "--build-jobs" if "--build-jobs" in command else "--jobs"
                    self.assertEqual(command[command.index(job_flag) + 1], "6")
                    # The pre-upgrade default rows use twelve; only the
                    # selected KE workspace/Engine rows explicitly use six.
                    default_jobs = "12"
                    self.assertEqual(ordinary[label][ordinary[label].index(job_flag) + 1], default_jobs)
                    self.assertIn("--frozen", command)
                    original = command.copy()
                    original[original.index(job_flag) + 1] = default_jobs
                    index = original.index("--features")
                    del original[index:index + 2]
                    self.assertEqual(original, ordinary[label])
        graph = VERIFY.commands("dependencies", schema_version=3,
                                patched_packages=patches, profile="engine-ke-association")
        self.assertEqual(hashlib.sha256(json.dumps(graph, separators=(",", ":")).encode()).hexdigest(),
                         "ed0422b68a97aa87d8641d270fa370feb013c47ebdf8fc63984018b8e6010fa0")

    def test_profile_still_requires_product_and_matching_policy_before_execution(self):
        import argparse
        for lane in ("portable", "windows"):
            args = argparse.Namespace(lane=lane, profile="engine-ke-association")
            with self.subTest(lane=lane), \
                    patch.object(VERIFY, "admit_product", side_effect=VERIFY.ENV.Incomplete("source admission refused")) as admission, \
                    patch.object(VERIFY.ENV, "run") as runner:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "source admission refused"):
                    VERIFY.execute(Path("/unused"), args, {})
                admission.assert_called_once()
                runner.assert_not_called()
            with self.subTest(lane=lane, policy="missing EP1"), \
                    patch.object(VERIFY, "admit_product", return_value=(FIXTURE.fixture_policy(), {})), \
                    patch.object(VERIFY, "admit_vendor") as vendor, patch.object(VERIFY.ENV, "run") as runner:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unadmitted dependency profile"):
                    VERIFY.execute(Path("/unused"), args, {})
                vendor.assert_not_called()
                runner.assert_not_called()
            with tempfile.TemporaryDirectory() as temporary:
                args.vendor_dir = Path(temporary)
                with self.subTest(lane=lane, admission="vendor"), \
                        patch.object(VERIFY, "admit_product", return_value=(prototype_policy(), {})), \
                        patch.object(VERIFY, "admit_vendor", side_effect=VERIFY.ENV.Incomplete("vendor admission refused")) as vendor, \
                        patch.object(VERIFY.ENV, "run") as runner:
                    with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "vendor admission refused"):
                        VERIFY.execute(Path("/unused"), args, {})
                    vendor.assert_called_once()
                    runner.assert_not_called()

    def test_exact_five_patch_cohort_admitted_foreign_sixth_duplicate_and_order_refuse(self):
        patches = [FIXTURE.fixture_path_patch(), *FIXTURE.fixture_accesskit_patches(),
                   *prototype_policy()["patched_packages"][1:]]
        expected = ("spin_on", "accesskit_windows", "accesskit_consumer",
                    "rustls-cng-crypto", "rustls-platform-verifier")
        self.assertEqual(PROFILE.patch_names({"patched_packages": patches}), expected)
        for changed in (patches + [{"name": "foreign"}], patches[:-1] + [patches[0]],
                        [patches[1], patches[0], *patches[2:]], patches + [patches[-1]]):
            with self.subTest(names=[item["name"] for item in changed]), self.assertRaises(PROFILE.Incomplete):
                PROFILE.patch_names({"patched_packages": changed})

    def test_default_execution_does_not_enter_prototype_row_selection(self):
        """Default execution retains its pre-EP1 flow after product admission."""
        import argparse
        with tempfile.TemporaryDirectory() as temporary:
            args = argparse.Namespace(lane="portable", vendor_dir=Path(temporary))
            with patch.object(VERIFY, "admit_product", return_value=({"schema_version": 2}, {})), \
                    patch.object(PROFILE, "profile_rows", side_effect=AssertionError("prototype selector entered")), \
                    patch.object(VERIFY, "admit_vendor", side_effect=VERIFY.ENV.Incomplete("vendor admission reached")), \
                    patch.object(VERIFY.ENV, "run") as runner:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "vendor admission reached"):
                    VERIFY.execute(Path("/unused"), args, {})
                runner.assert_not_called()




def prototype_policy():
    """Independent tiny graph; synthetic source pins confer no execution grant."""
    policy = FIXTURE.add_patch(FIXTURE.fixture_policy())
    for name, version in (("aes-siv", "0.8.0"), ("aes", "0.9.3"), ("cmac", "0.8.0"),
                          ("rustls-pki-types", "1.15.1"), ("zeroize", "1.9.1"), ("rustls", "0.23.46")):
        policy["packages"].append({"name": name, "version": version, "checksum": "a" * 64,
            "license": "MIT", "selected_license": "MIT", "build_script": False, "proc_macro": False,
            "links": None, "files_sha256": "b" * 64, "upstream_revision": None})
    policy["packages"].sort(key=PROFILE.identity)
    for name in PROFILE.EP1_PATCH_NAMES:
        policy["patched_packages"].append({**PROFILE.EP1_PATCH_UPSTREAM[name], "upstream_files_sha256": "c" * 64,
            "files_sha256": "d" * 64,
            "modified_files_sha256": {path: "e" * 64 for path in PROFILE.EP1_MODIFIED_FILES[name]}})
    rows = copy.deepcopy(policy["rows"])
    for target, row in rows.items():
        crypto = {"aes-siv@0.8.0": {"target": [["zeroize"]]}, "aes@0.9.3": {"target": [["zeroize"]]},
            "cmac@0.8.0": {"target": [["zeroize"]]}, "rustls-pki-types@1.15.1": {"target": [["alloc", "std"]]},
            "zeroize@1.9.1": {"target": [["alloc", "default"]]}}
        if target == FIXTURE.WINDOWS:
            crypto.update({"rustls@0.23.46": {"target": [["std"]]},
                "rustls-cng-crypto@0.1.2": {"target": [[]]}, "rustls-platform-verifier@0.7.1": {"target": [[]]}})
        row["features"].update(crypto)
        row["metadata_features"].update({key: roles["target"][0] for key, roles in crypto.items()})
        row["local_features"]["nevrotick-engine@0.0.0"] = {"target": [["ke-association-prototype"]]}
        row["local_features"]["nevrotick-s115-aes-siv-fixture@0.0.0"] = {"target": [[]]}
        engine = row["package_scopes"]["nevrotick-engine"]
        engine["features"] = copy.deepcopy(crypto)
        engine["local_features"]["nevrotick-engine@0.0.0"] = {"target": [["ke-association-prototype"]]}
        engine["local_features"]["nevrotick-s115-aes-siv-fixture@0.0.0"] = {"target": [[]]}
    policy["engine_ke_association"] = {"rows": rows, "first_party_dependency": {
        "name": "nevrotick-s115-aes-siv-fixture", "version": "0.0.0", "path": "tools/crypto-aes-siv-witness",
        "license": "GPL-2.0-only", "selected_license": "GPL-2.0-only", "build_script": False,
        "proc_macro": False, "links": None, "files_sha256": "f" * 64, "manifest_sha256": "1" * 64}}
    return policy


def prototype_metadata(policy, target):
    """Build actual-format metadata with a nonmember local node and exact feature."""
    selected = copy.deepcopy(policy)
    selected["rows"] = selected["engine_ke_association"]["rows"]
    value = FIXTURE.fixture_metadata(selected, target)
    aes_id = "path+file:///work/fixture/tools/crypto-aes-siv-witness#0.0.0"
    value["packages"].append({"id": aes_id, "name": "nevrotick-s115-aes-siv-fixture", "version": "0.0.0",
        "source": None, "license": "GPL-2.0-only", "manifest_path": "/work/fixture/tools/crypto-aes-siv-witness/Cargo.toml",
        "targets": [{"kind": ["lib"]}], "links": None, "dependencies": [], "features": {}})
    value["resolve"]["nodes"].append({"id": aes_id, "features": [], "dependencies": [], "deps": []})
    engine = next(p for p in value["packages"] if p["name"] == "nevrotick-engine")
    node = next(n for n in value["resolve"]["nodes"] if n["id"] == engine["id"])
    node["features"] = ["ke-association-prototype"]
    engine["features"] = {"ke-association-prototype": []}
    children = set(selected["rows"][target]["package_scopes"]["nevrotick-engine"]["features"]) | {"nevrotick-s115-aes-siv-fixture@0.0.0"}
    for child in value["packages"]:
        if PROFILE.identity(child) not in children:
            continue
        alias = child["name"].replace("-", "_")
        node["dependencies"].append(child["id"])
        node["deps"].append({"name": alias, "pkg": child["id"], "dep_kinds": [{"kind": None, "target": None}]})
        engine["dependencies"].append({"name": child["name"], "rename": None, "req": "=" + child["version"],
            "kind": None, "target": None, "optional": True, "uses_default_features": False, "features": []})
        engine["features"]["ke-association-prototype"].append("dep:" + child["name"])
    return value


def prototype_trees(policy, target):
    row = policy["engine_ke_association"]["rows"][target]
    engine = "nevrotick-engine v0.0.0 (/work/fixture/crates/nevrotick-engine)|ke-association-prototype\n"
    for name in ("ntp-wire", "time-domain"):
        engine += "|-- " + name + " v0.0.0 (/work/fixture/crates/" + name + ")|\n"
    engine += "|-- nevrotick-s115-aes-siv-fixture v0.0.0 (/work/fixture/tools/crypto-aes-siv-witness)|\n"
    for key, roles in sorted(row["package_scopes"]["nevrotick-engine"]["features"].items()):
        name, version = key.split("@")
        annotation = " (/work/fixture/" + PROFILE.EP1_PATCH_UPSTREAM[name]["path"] + ")" if name in PROFILE.EP1_PATCH_NAMES else ""
        engine += "|-- " + name + " v" + version + annotation + "|" + ",".join(roles["target"][0]) + "\n"
    workspace = FIXTURE.patch_tree(FIXTURE.fixture_tree(target)).replace(
        "nevrotick-engine v0.0.0 (/work/fixture/crates/nevrotick-engine)|\n", engine)
    scopes = FIXTURE.fixture_package_trees(target)
    scopes["nevrotick-ui"] = FIXTURE.patch_tree(scopes["nevrotick-ui"])
    scopes["spin_on"] = "spin_on v0.1.1 (/work/fixture/third-party/spin_on)|\n"
    scopes["nevrotick-engine"] = engine
    return workspace, scopes


class EngineKeAdmissionControls(unittest.TestCase):
    def setUp(self):
        age = patch.object(PROFILE.COOLDOWN, "validate_policy_admission", return_value=None)
        age.start()
        self.addCleanup(age.stop)
        self.policy = prototype_policy()
        self.pin = patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(self.policy))
        self.pin.start()
        self.addCleanup(self.pin.stop)

    def test_prototype_and_default_rows_have_distinct_healthy_consumers(self):
        PROFILE.validate_policy(self.policy)
        import json
        for target in (FIXTURE.HOST, FIXTURE.WINDOWS):
            with self.subTest(target=target):
                metadata = prototype_metadata(self.policy, target)
                workspace, scopes = prototype_trees(self.policy, target)
                PROFILE.validate_metadata(target, json.dumps(metadata), self.policy, profile="engine-ke-association")
                PROFILE.validate_tree(target, workspace, self.policy, profile="engine-ke-association")
                projection = PROFILE.selected_metadata(target, json.dumps(metadata), workspace, self.policy,
                    package_trees=scopes, profile="engine-ke-association")
                self.assertEqual(len(projection["workspace_members"]), 7)
                self.assertTrue(any(p["name"] == "nevrotick-s115-aes-siv-fixture" for p in projection["packages"]))
                PROFILE.validate_tree(target, FIXTURE.patch_tree(FIXTURE.fixture_tree(target)), self.policy)

    def test_shape_rejects_default_leak_foreign_path_feature_and_workspace_promotion(self):
        for change in ("default-leak", "foreign-path", "foreign-feature", "provider-host", "gnu-tls", "foreign-profile"):
            value = copy.deepcopy(self.policy)
            row = value["engine_ke_association"]["rows"][FIXTURE.WINDOWS]
            if change == "default-leak":
                value["rows"][FIXTURE.WINDOWS]["features"]["zeroize@1.9.1"] = {"target": [["alloc"]]}
                value["rows"][FIXTURE.WINDOWS]["metadata_features"]["zeroize@1.9.1"] = ["alloc"]
            elif change == "foreign-path": value["engine_ke_association"]["first_party_dependency"]["path"] = "tools/foreign"
            elif change == "foreign-feature": row["local_features"]["nevrotick-engine@0.0.0"] = {"target": [["default", "ke-association-prototype"]]}
            elif change == "provider-host": row["features"]["rustls-cng-crypto@0.1.2"] = {"host": [[]]}
            elif change == "gnu-tls": value["engine_ke_association"]["rows"][FIXTURE.HOST]["features"]["rustls@0.23.46"] = {"target": [["std"]]}
            else: value["engine_ke_association"]["foreign"] = {}
            with self.subTest(change=change), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(value)

    def test_actual_metadata_and_tree_cannot_promote_or_relabel_nonmember(self):
        import json
        healthy = prototype_metadata(self.policy, FIXTURE.HOST)
        workspace, _ = prototype_trees(self.policy, FIXTURE.HOST)
        for change in ("member", "path", "feature", "grant"):
            value = copy.deepcopy(healthy)
            aes = next(p for p in value["packages"] if p["name"] == "nevrotick-s115-aes-siv-fixture")
            if change == "member": value["workspace_members"].append(aes["id"])
            elif change == "path": aes["manifest_path"] = "/work/fixture/tools/foreign/Cargo.toml"
            elif change == "feature": next(n for n in value["resolve"]["nodes"] if n["id"] == aes["id"])["features"] = ["forged"]
            else: aes["license"] = "MIT"
            with self.subTest(change=change), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_metadata(FIXTURE.HOST, json.dumps(value), self.policy, profile="engine-ke-association")
        with self.assertRaises(PROFILE.Incomplete):
            PROFILE.validate_tree(FIXTURE.HOST, workspace.replace("tools/crypto-aes-siv-witness", "tools/foreign"), self.policy, profile="engine-ke-association")

    def test_bad_patch_and_unrelated_scope_cannot_acquire_provider(self):
        for change in ("path", "revision", "features", "unrelated"):
            value = copy.deepcopy(self.policy)
            record = value["patched_packages"][-1]
            if change == "path": record["path"] = "tools/foreign"
            elif change == "revision": record["upstream_revision"] = "0" * 40
            elif change == "features": value["engine_ke_association"]["rows"][FIXTURE.WINDOWS]["features"]["rustls-platform-verifier@0.7.1"] = {"target": [["default"]]}
            else: value["engine_ke_association"]["rows"][FIXTURE.WINDOWS]["package_scopes"]["time-core"]["features"]["rustls-cng-crypto@0.1.2"] = {"target": [[]]}
            with self.subTest(change=change), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(value)


class EngineKeSourceControls(unittest.TestCase):
    def test_exact_nonmember_source_is_bound_and_cannot_change_after_admission(self):
        import hashlib
        import json
        import shutil
        import tempfile
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / "tools/crypto-aes-siv-witness"
            directory.mkdir(parents=True)
            original = FIXTURE.ROOT / "tools/crypto-aes-siv-witness"
            for name in ("Cargo.toml", "lib.rs", "sealing.rs", "staged.rs"):
                shutil.copyfile(original / name, directory / name)
            record = prototype_policy()["engine_ke_association"]["first_party_dependency"]
            files = VERIFY.inventory(directory)
            record["files_sha256"] = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
            record["manifest_sha256"] = files["Cargo.toml"]
            accepted = VERIFY.admit_first_party_dependency(root, record)
            self.assertIn("tools/crypto-aes-siv-witness/lib.rs", accepted)
            (directory / "sealing.rs").write_text("changed source")
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "nonmember source"):
                VERIFY.admit_first_party_dependency(root, record)

    def test_prototype_deny_view_preserves_default_and_adds_exact_selected_grants(self):
        import tomllib
        policy = prototype_policy()
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)), \
                patch.object(PROFILE.COOLDOWN, "validate_policy_admission", return_value=None):
            base = (FIXTURE.ROOT / "deny.toml").read_text()
            default = tomllib.loads(VERIFY.product_deny(base, policy, FIXTURE.WINDOWS))
            prototype = tomllib.loads(VERIFY.product_deny(base, policy, FIXTURE.WINDOWS, profile="engine-ke-association"))
            first = {item["crate"] for item in default["licenses"]["exceptions"]}
            second = {item["crate"] for item in prototype["licenses"]["exceptions"]}
            self.assertNotIn("rustls-cng-crypto@0.1.2", first)
            self.assertIn("rustls-cng-crypto@0.1.2", second)
            self.assertNotIn("ring", str(prototype["licenses"]["exceptions"]))
            self.assertEqual(default["bans"]["deny"], prototype["bans"]["deny"])


class EngineKeExecutionAccountingControls(unittest.TestCase):
    """Synthetic JUnit admission controls, not executed Rust test evidence."""

    REQUIRED_BY_SOURCE = {
        "tests.rs": ["configuration_admission_and_typed_identity",
                     "strict_deadline_neighbors_and_fixed_expiry"],
        "process.rs": ["claim_is_permanent_before_and_after_provider_scope"],
        "association/tests.rs": ["genuine_owner_boundary_moves_cookies_and_consumes_completion_once",
                                 "foreign_completion_refuses_without_partial_publication",
                                 "post_ke_cancellation_refuses_before_entropy",
                                 "association_expiry_after_ke_prevents_preview",
                                 "published_association_reaches_actual_sealer_and_finishes_once",
                                 "preview_refusal_does_not_reset_work_credit"],
        "request_tests.rs": ["complete_request_matches_existing_independent_answer",
                             "s2c_changes_do_not_select_the_outgoing_key"],
        "handoff/tests.rs": ["cancellation_after_sealing_refuses_final_completion"],
    }

    @classmethod
    def expected_names(cls):
        """Independently name required real unit tests across the changed owners."""
        modules = {"tests.rs": "tests", "process.rs": "process::tests",
                   "association/tests.rs": "association::tests", "request_tests.rs": "request::tests",
                   "handoff/tests.rs": "handoff::tests"}
        return {"nts_ke_experiment::" + modules[path] + "::" + name
                for path, names in cls.REQUIRED_BY_SOURCE.items() for name in names}

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.junit = self.work / "target/nextest/default/junit.xml"
        self.junit.parent.mkdir(parents=True)

    def junit_with(self, names, *, unit_suite="nevrotick-engine", outcome=None):
        """Retain required old Engine neighbors beside selected KE fixture names."""
        cases = "".join('<testcase name="' + name + '">' +
                        ('<' + outcome + '/>' if outcome else '') + '</testcase>' for name in sorted(names))
        self.junit.write_text('<testsuites><testsuite name="nevrotick-engine">'
            '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control"/>'
            '</testsuite><testsuite name="' + unit_suite + '">' + cases +
            '</testsuite><testsuite name="nevrotick-engine::prepare_entered">'
            '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
            '</testsuite></testsuites>')

    def test_ke_discriminators_exist_in_actual_migrated_source(self):
        self.assertEqual(VERIFY.ENGINE_KE_TEST_CASES, self.expected_names())
        for relative, names in self.REQUIRED_BY_SOURCE.items():
            source = (FIXTURE.ROOT / "crates/nevrotick-engine/src/nts_ke_experiment" / relative).read_text()
            for name in names:
                with self.subTest(source=relative, name=name):
                    self.assertIn("fn " + name + "(", source)

    def test_feature_report_accepts_required_real_unit_names_in_both_profiles(self):
        self.junit_with(self.expected_names())
        for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
            with self.subTest(lane=lane):
                result = VERIFY.validate_test_result(lane, "", self.work, profile="engine-ke-association")
                self.assertEqual(result["executed"], len(self.expected_names()) + 2)

    def test_old_report_or_each_missing_ke_control_cannot_qualify_feature(self):
        for names in (set(), *(self.expected_names() - {name} for name in sorted(self.expected_names()))):
            self.junit_with(names)
            for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
                with self.subTest(lane=lane, missing=self.expected_names() - names), \
                        self.assertRaisesRegex(VERIFY.ENV.Incomplete, "KE.*unit"):
                    VERIFY.validate_test_result(lane, "", self.work, profile="engine-ke-association")
        # The original no-feature row continues to admit its own healthy report.
        self.junit_with(set())
        self.assertEqual(VERIFY.validate_test_result("product-tests:nevrotick-engine", "", self.work)["executed"], 2)

    def test_names_from_other_binary_or_skipped_cases_cannot_stand_in_for_units(self):
        for suite, outcome in (("nevrotick-engine::unrelated", None), ("foreign", None),
                               ("nevrotick-engine", "skipped"), ("nevrotick-engine", "failure"),
                               ("nevrotick-engine", "error")):
            self.junit_with(self.expected_names(), unit_suite=suite, outcome=outcome)
            for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
                with self.subTest(lane=lane, suite=suite, outcome=outcome), self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.validate_test_result(lane, "", self.work, profile="engine-ke-association")

    def test_foreign_test_profile_is_not_silently_defaulted(self):
        self.junit_with(self.expected_names())
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.validate_test_result("product-tests:nevrotick-engine", "", self.work, profile="foreign")


if __name__ == "__main__":
    unittest.main()
