"""Inert schema-3 admission controls; no Cargo, dependency or native execution."""
import argparse
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("slint_profile_verify_control", ROOT / "scripts/verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)
PROFILE = VERIFY.SLINT
HOST = "x86_64-unknown-linux-gnu"
WINDOWS = "x86_64-pc-windows-msvc"
MEMBERS = ("nevrotick-ui", "ntp-wire", "time-domain", "ntp-exchange", "time-core", "nevrotick-engine", "windows-platform")
RUNTIME_FEATURES = ["accessibility", "backend-winit", "compat-1-18", "renderer-software", "std", "unstable-winit-030"]
HOST_FEATURES = ["compat-1-18", "renderer-software", "std"]
LOCAL_CLOSURES = {"nevrotick-ui": {"nevrotick-ui"}, "ntp-wire": {"ntp-wire"}, "time-domain": {"time-domain"},
                  "time-core": {"time-core", "time-domain"},
                  "nevrotick-engine": {"nevrotick-engine", "ntp-wire", "time-domain"},
                  "windows-platform": {"windows-platform"},
                  "ntp-exchange": {"ntp-exchange", "ntp-wire", "time-domain", "time-core"}}


def fixture_policy():
    """Independent tiny exact graph with split roles, duplicate versions and an inert GPU package."""
    selected = []
    for name, version, grant, build, macro in (
        ("slint", "1.18.1", "GPL-3.0-only", False, False),
        ("slint-build", "1.18.1", "GPL-3.0-only", False, False),
        ("helper", "1.0.0", "MIT", True, False),
        ("helper", "2.0.0", "Apache-2.0", False, False),
        ("macro-control", "1.0.0", "MIT", False, True),
    ):
        selected.append({"name": name, "version": version, "checksum": "a" * 64,
                         "license": grant, "selected_license": grant, "build_script": build,
                         "proc_macro": macro, "links": None, "files_sha256": "b" * 64,
                         "upstream_revision": None})
    selected.sort(key=lambda item: item["name"] + "@" + item["version"])
    features = {"slint@1.18.1": {"target": [RUNTIME_FEATURES.copy()]},
                "slint-build@1.18.1": {"host": [["compat-1-18"]]},
                "helper@1.0.0": {"target": [["narrow"]], "host": [["narrow"]]},
                "helper@2.0.0": {"target": [[]]},
                "macro-control@1.0.0": {"host": [[]]}}
    metadata_features = {key: sorted({feature for sets in roles.values() for values in sets for feature in values})
                         for key, roles in features.items()}
    metadata_features["skia-bindings@1.0.0"] = ["metadata-gpu-only"]
    row = {"features": features,
           "local_features": {name + "@0.0.0": {"target": [[]]} for name in MEMBERS},
           "metadata_features": metadata_features,
           "allowed_build_scripts": ["helper@1.0.0", "nevrotick-ui@0.0.0"],
           "allowed_proc_macros": ["macro-control@1.0.0"],
           "duplicate_exceptions": ["helper@1.0.0", "helper@2.0.0"],
           "package_scopes": {name: {"features": copy.deepcopy(features) if name == "nevrotick-ui" else {},
                                      "local_features": {member + "@0.0.0": {"target": [[]]} for member in local}}
                              for name, local in LOCAL_CLOSURES.items()}}
    rows = {HOST: copy.deepcopy(row), WINDOWS: copy.deepcopy(row)}
    rows[HOST]["features"]["slint@1.18.1"] = {"target": [HOST_FEATURES.copy()]}
    rows[HOST]["package_scopes"]["nevrotick-ui"]["features"]["slint@1.18.1"] = {"target": [HOST_FEATURES.copy()]}
    return {"schema_version": 3, "scope": "Inert independently specified schema-3 control; never compiled",
            "lockfile_sha256": "c" * 64,
            "manifests_sha256": {name: "d" * 64 for name in
                ("Cargo.toml", *("crates/" + member + "/Cargo.toml" for member in MEMBERS))},
            "first_party_build_scripts": {"crates/nevrotick-ui/build.rs": "e" * 64},
            "packages": selected,
            "metadata_only_packages": [{"name": "skia-bindings", "version": "1.0.0", "checksum": "f" * 64,
                                         "license": "MIT", "files_sha256": "0" * 64, "role": "metadata-only"}],
            "rows": rows}


def fixture_path_patch():
    """Independent upstream pins; derived bytes in this graph-only fixture are inert."""
    return {"name": "spin_on", "version": "0.1.1", "path": "third-party/spin_on",
            "license": "Apache-2.0 OR MIT", "selected_license": "MIT", "build_script": False,
            "proc_macro": False, "links": None,
            "upstream_checksum": "076e103ed41b9864aa838287efe5f4e3a7a0362dd00671ae62a212e5e4612da2",
            "upstream_files_sha256": "a4c380640062932da27aaa676898ba8977e846e0e90b8100a0b1a03e6da82e2c",
            "upstream_revision": "1605b63506316d89722c650e3fac6fdb643c3ef9", "files_sha256": "1" * 64,
            "modified_files_sha256": {name: "2" * 64 for name in
                ("Cargo.toml", "src/lib.rs", "NEVROTICK-PATCH.md", "tests/stack_pin.rs", "Cargo.lock", ".gitignore")}}


def add_patch(policy):
    policy["patched_packages"] = [fixture_path_patch()]
    for row in policy["rows"].values():
        row["features"]["spin_on@0.1.1"] = {"host": [[]]}
        row["metadata_features"]["spin_on@0.1.1"] = []
        row["package_scopes"]["nevrotick-ui"]["features"]["spin_on@0.1.1"] = {"host": [[]]}
        row["package_scopes"]["spin_on"] = {"features": {"spin_on@0.1.1": {"target": [[]]}}, "local_features": {}}
    return policy


def fixture_accesskit_patches():
    """Independent exact upstream identities; synthetic derivative hashes grant no execution."""
    return [
        {"name": "accesskit_windows", "version": "0.34.0", "path": "third-party/accesskit_windows",
         "license": "MIT AND BSD-3-Clause AND GPL-3.0-only",
         "selected_license": "MIT AND BSD-3-Clause AND GPL-3.0-only", "build_script": False,
         "proc_macro": False, "links": None,
         "upstream_checksum": "106c2b961215864d1c2e703ee63269c25c4e80a577ffb2c1017b9c17dcdf83a1",
         "upstream_files_sha256": "7b5aa35d98dbf1c4ba0d311f591de9486000b84f97190462de4266542d0306e7",
         "upstream_revision": "c88605b96d04431f9c3c792464a0f2f253480e94", "files_sha256": "3" * 64,
         "modified_files_sha256": {name: "4" * 64 for name in
             ("Cargo.toml", "NEVROTICK-PATCH.md", "src/lib.rs", "src/node.rs", "src/range_state.rs",
              "src/text.rs", "src/value_input.rs")}},
        {"name": "accesskit_consumer", "version": "0.38.0", "path": "third-party/accesskit_consumer",
         "license": "(MIT OR Apache-2.0) AND GPL-3.0-only", "selected_license": "MIT AND GPL-3.0-only",
         "build_script": False, "proc_macro": False, "links": None,
         "upstream_checksum": "5d10a236f96f87d70732e44520046785431ef01d5bcd6b041317bfadd2f88245",
         "upstream_files_sha256": "fc9e9ee737b62b4b529977e0dc67e6991e9791995d2659bfb31b36282ea7e784",
         "upstream_revision": "c88605b96d04431f9c3c792464a0f2f253480e94", "files_sha256": "5" * 64,
         "modified_files_sha256": {name: "6" * 64 for name in
             ("Cargo.toml", "NEVROTICK-PATCH.md", "src/text.rs", "src/text/nevrotick_word_navigation_tests.rs")}},
    ]


def add_accesskit_patches(policy):
    add_patch(policy)
    policy["patched_packages"].extend(fixture_accesskit_patches())
    row = policy["rows"][WINDOWS]
    for key in ("accesskit_windows@0.34.0", "accesskit_consumer@0.38.0"):
        row["features"][key] = {"target": [[]]}
        row["metadata_features"][key] = []
        row["package_scopes"]["nevrotick-ui"]["features"][key] = {"target": [[]]}
    policy["rows"][HOST]["metadata_features"]["accesskit_consumer@0.38.0"] = []
    return policy


def accesskit_patch_tree(output, target=HOST):
    output = patch_tree(output)
    if target == WINDOWS:
        line = "|-- slint v1.18.1|" + ",".join(RUNTIME_FEATURES) + "\n"
        output = output.replace(line, line +
            "|   `-- accesskit_windows v0.34.0 (/work/fixture/third-party/accesskit_windows)|\n"
            "|       `-- accesskit_consumer v0.38.0 (/work/fixture/third-party/accesskit_consumer)|\n")
    return output


def patch_tree(output):
    return output.replace("|-- slint-build v1.18.1|compat-1-18\n",
                          "|-- slint-build v1.18.1|compat-1-18\n"
                          "|   `-- spin_on v0.1.1 (/work/fixture/third-party/spin_on)|\n")


def fixture_tree(target=HOST):
    """Small real-format Cargo tree with ordinary, build and proc-macro paths."""
    runtime = RUNTIME_FEATURES if target == WINDOWS else HOST_FEATURES
    return ("nevrotick-ui v0.0.0 (/work/fixture/crates/nevrotick-ui)|\n"
            "|-- slint v1.18.1|" + ",".join(runtime) + "\n"
            "|-- helper v1.0.0|narrow\n"
            "|-- helper v2.0.0|\n"
            "`-- macro-control v1.0.0 (proc-macro)|\n"
            "[build-dependencies]\n"
            "|-- slint-build v1.18.1|compat-1-18\n"
            "`-- helper v1.0.0|narrow\n" +
            "".join(name + " v0.0.0 (/work/fixture/crates/" + name + ")|\n" for name in MEMBERS[1:]))


def fixture_package_trees(target=HOST):
    """Independent package roots include the one admitted local dev edge."""
    def root(name):
        return name + " v0.0.0 (/work/fixture/crates/" + name + ")|\n"

    return {"nevrotick-ui": fixture_tree(target).split("ntp-wire v0.0.0")[0],
            "ntp-wire": root("ntp-wire"), "time-domain": root("time-domain"),
            "windows-platform": root("windows-platform"),
            "time-core": root("time-core") + "`-- " + root("time-domain"),
            "nevrotick-engine": root("nevrotick-engine") + "|-- " + root("ntp-wire") + "`-- " + root("time-domain"),
            "ntp-exchange": root("ntp-exchange") + "|-- " + root("ntp-wire") + "`-- " + root("time-domain") +
                            "[dev-dependencies]\n`-- " + root("time-core") + "    `-- " + root("time-domain")}


def fixture_metadata(policy, target=HOST):
    """Overapproximation includes an unused GPU edge, which the projection must remove."""
    packages, nodes = [], []
    for record in policy["packages"] + policy["metadata_only_packages"] + policy.get("patched_packages", []):
        key = record["name"] + "@" + record["version"]
        if key not in policy["rows"][target]["metadata_features"]:
            continue
        package_id = "registry+https://github.com/rust-lang/crates.io-index#" + key
        targets = [{"kind": ["proc-macro" if record.get("proc_macro") else "lib"]}]
        if record.get("build_script"):
            targets.append({"kind": ["custom-build"]})
        source = "registry+https://github.com/rust-lang/crates.io-index"
        manifest = "/vendor/" + record["name"] + "-" + record["version"] + "/Cargo.toml"
        if "path" in record:
            source = None
            manifest = "/work/fixture/" + record["path"] + "/Cargo.toml"
            package_id = "path+file:///work/fixture/" + record["path"] + "#" + record["version"]
        packages.append({"id": package_id, "name": record["name"], "version": record["version"],
                         "source": source, "license": record["license"], "manifest_path": manifest,
                         "targets": targets, "links": record.get("links"), "dependencies": [],
                         "features": {feature: [] for feature in policy["rows"][target]["metadata_features"][key]}})
        nodes.append({"id": package_id, "features": policy["rows"][target]["metadata_features"][key],
                      "dependencies": [], "deps": []})
    locals_ = []
    for name in MEMBERS:
        package_id = "path+file:///work/fixture/crates/" + name + "#0.0.0"
        locals_.append(package_id)
        packages.append({"id": package_id, "name": name, "version": "0.0.0", "source": None,
                         "license": "GPL-3.0-only" if name == "nevrotick-ui" else "GPL-2.0-only",
                         "dependencies": [], "features": {}})
        nodes.append({"id": package_id, "features": [], "dependencies": [], "deps": []})
    by_key = {package["name"] + "@" + package["version"]: package for package in packages}
    by_id = {node["id"]: node for node in nodes}

    def edge(parent, child, kind=None, *, rename=None, optional=False):
        before, after = by_key[parent], by_key[child]
        alias = rename or after["name"]
        before["dependencies"].append({"name": after["name"], "rename": rename, "req": "=" + after["version"],
                                       "kind": kind, "target": None, "optional": optional,
                                       "uses_default_features": False, "features": []})
        node = by_id[before["id"]]
        existing = next((item for item in node["deps"] if item["name"] == alias.replace("-", "_")
                         and item["pkg"] == after["id"]), None)
        if existing is None:
            existing = {"name": alias.replace("-", "_"), "pkg": after["id"], "dep_kinds": []}
            node["deps"].append(existing)
        existing["dep_kinds"].append({"kind": kind, "target": None})
        node["dependencies"] = sorted({item["pkg"] for item in node["deps"]})

    ui = "nevrotick-ui@0.0.0"
    edge(ui, "slint@1.18.1")
    edge(ui, "slint-build@1.18.1", "build")
    edge(ui, "helper@1.0.0", rename="helper_one")
    edge(ui, "helper@1.0.0", "build", rename="helper_one")
    edge(ui, "helper@2.0.0", rename="helper_two")
    edge(ui, "macro-control@1.0.0")
    edge(ui, "skia-bindings@1.0.0", optional=True)
    edge("time-core@0.0.0", "time-domain@0.0.0")
    edge("nevrotick-engine@0.0.0", "ntp-wire@0.0.0")
    edge("nevrotick-engine@0.0.0", "time-domain@0.0.0")
    edge("ntp-exchange@0.0.0", "ntp-wire@0.0.0")
    edge("ntp-exchange@0.0.0", "time-domain@0.0.0")
    edge("ntp-exchange@0.0.0", "time-core@0.0.0", "dev")
    if policy.get("patched_packages"):
        edge("slint-build@1.18.1", "spin_on@0.1.1")
    if "accesskit_windows@0.34.0" in by_key:
        edge("slint@1.18.1", "accesskit_windows@0.34.0")
    if "accesskit_consumer@0.38.0" in by_key:
        if "accesskit_windows@0.34.0" in by_key:
            edge("accesskit_windows@0.34.0", "accesskit_consumer@0.38.0")
        else:
            edge("slint@1.18.1", "accesskit_consumer@0.38.0", optional=True)
    return {"packages": packages, "resolve": {"nodes": nodes, "root": None},
            "workspace_members": locals_, "workspace_default_members": locals_, "workspace_root": "/work/fixture"}


class SlintProfileControls(unittest.TestCase):
    def setUp(self):
        self.policy = fixture_policy()
        self.pin = patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(self.policy))
        self.pin.start()
        self.addCleanup(self.pin.stop)

    def test_reviewed_shape_and_independent_code_pin_are_both_required(self):
        PROFILE.validate_policy(self.policy)
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", None):
            with self.assertRaisesRegex(PROFILE.Incomplete, "code-owned pin"):
                PROFILE.validate_policy(self.policy)
        changed = copy.deepcopy(self.policy)
        changed["scope"] += " tampered"
        PROFILE.validate_policy_shape(changed)
        with self.assertRaisesRegex(PROFILE.Incomplete, "code-owned pin"):
            PROFILE.validate_policy(changed)

    def test_public_winit_feature_remains_exact_windows_target_only(self):
        self.assertEqual(HOST_FEATURES, ["compat-1-18", "renderer-software", "std"])
        self.assertEqual(RUNTIME_FEATURES, ["accessibility", "backend-winit", "compat-1-18",
                                          "renderer-software", "std", "unstable-winit-030"])
        PROFILE.validate_policy(self.policy)
        cases = []
        missing = copy.deepcopy(self.policy)
        missing["rows"][WINDOWS]["features"]["slint@1.18.1"]["target"][0].remove("unstable-winit-030")
        cases.append(missing)
        extra = copy.deepcopy(self.policy)
        extra["rows"][WINDOWS]["features"]["slint@1.18.1"]["target"][0].append("system-testing")
        extra["rows"][WINDOWS]["features"]["slint@1.18.1"]["target"][0].sort()
        extra["rows"][WINDOWS]["metadata_features"]["slint@1.18.1"].append("system-testing")
        extra["rows"][WINDOWS]["metadata_features"]["slint@1.18.1"].sort()
        cases.append(extra)
        duplicate = copy.deepcopy(self.policy)
        duplicate["rows"][WINDOWS]["features"]["slint@1.18.1"]["target"][0].append("unstable-winit-030")
        cases.append(duplicate)
        linux = copy.deepcopy(self.policy)
        linux["rows"][HOST]["features"]["slint@1.18.1"]["target"][0].append("unstable-winit-030")
        cases.append(linux)
        host_role = copy.deepcopy(self.policy)
        host_role["rows"][WINDOWS]["features"]["slint@1.18.1"]["host"] = [["unstable-winit-030"]]
        cases.append(host_role)
        for index, changed in enumerate(cases):
            with self.subTest(index=index), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)
        PROFILE.validate_policy(self.policy)

    def test_canonical_digest_is_independent_of_object_key_order(self):
        reordered = dict(reversed(list(self.policy.items())))
        self.assertEqual(PROFILE.canonical_policy_sha256(reordered), PROFILE.canonical_policy_sha256(self.policy))

    def test_unknown_fields_and_role_promotion_are_rejected_before_the_pin(self):
        for variant in ("top", "record", "metadata", "row", "promote", "duplicate"):
            policy = copy.deepcopy(self.policy)
            if variant == "top": policy["expected_sha256"] = "a" * 64
            elif variant == "record": policy["packages"][0]["role"] = "selected"
            elif variant == "metadata": policy["metadata_only_packages"][0]["build_script"] = True
            elif variant == "row": policy["rows"][HOST]["allow_everything"] = True
            elif variant == "promote": policy["packages"].append(copy.deepcopy(policy["metadata_only_packages"][0]))
            else: policy["packages"].insert(0, copy.deepcopy(policy["packages"][0]))
            with self.subTest(variant=variant), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(policy)

    def test_identity_hash_grant_and_feature_shapes_fail_closed(self):
        mutations = (
            lambda p: p.update(schema_version=True),
            lambda p: p["packages"][0].update(version="1.0.0/../../escape"),
            lambda p: p["packages"][0].update(checksum="A" * 64),
            lambda p: p["packages"][0].update(build_script=1),
            lambda p: p["packages"][0].update(selected_license="GPL-3.0-only"),
            lambda p: p["packages"][0].update(upstream_revision="main"),
            lambda p: p["rows"][HOST]["features"]["slint@1.18.1"].update(target=[["default"]]),
            lambda p: p["rows"][HOST]["features"]["helper@1.0.0"].update(host=[["narrow", "narrow"]]),
            lambda p: p["rows"][HOST]["features"]["helper@1.0.0"].update(host=[["outside-metadata"]]),
            lambda p: p["rows"][HOST]["features"].update({"skia-bindings@1.0.0": {"target": [[]]}}),
            lambda p: p["rows"][HOST]["local_features"].pop("time-core@0.0.0"),
            lambda p: p["rows"][HOST]["allowed_build_scripts"].append("skia-bindings@1.0.0"),
            lambda p: p["rows"][HOST].update(duplicate_exceptions=["helper"]),
            lambda p: p["rows"][HOST]["features"]["macro-control@1.0.0"].update(target=[[]]),
            lambda p: p["rows"][HOST]["package_scopes"].pop("time-core"),
            lambda p: p["rows"][HOST]["package_scopes"]["ntp-exchange"]["local_features"].pop("time-core@0.0.0"),
            lambda p: p["rows"][HOST]["package_scopes"]["time-core"]["features"].update({"slint@1.18.1": {"target": [HOST_FEATURES]}}),
            lambda p: p["rows"][HOST]["package_scopes"]["nevrotick-ui"]["features"]["helper@1.0.0"].update(target=[["outside-profile"]]),
        )
        for index, mutate in enumerate(mutations):
            policy = copy.deepcopy(self.policy)
            mutate(policy)
            with self.subTest(index=index), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(policy)

    def test_exact_duplicate_versions_do_not_merge(self):
        PROFILE.validate_policy(self.policy)
        self.assertEqual(PROFILE.duplicate_identities(self.policy["rows"][HOST]["features"]),
                         ["helper@1.0.0", "helper@2.0.0"])

    def test_single_path_patch_has_separate_provenance_and_standalone_scope(self):
        policy = add_patch(copy.deepcopy(self.policy))
        trees = {name: patch_tree(output) for name, output in fixture_package_trees().items()}
        trees["spin_on"] = "spin_on v0.1.1 (/work/fixture/third-party/spin_on)|\n"
        data = fixture_metadata(policy)
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)):
            PROFILE.validate_policy(policy)
            PROFILE.validate_tree(HOST, trees["spin_on"], policy, scope="spin_on")
            projected = PROFILE.selected_metadata(HOST, json.dumps(data), patch_tree(fixture_tree()), policy,
                                                  package_trees=trees)
            self.assertEqual(len(projected["workspace_members"]), 7)
            patch_pkg = next(package for package in projected["packages"] if package["name"] == "spin_on")
            self.assertIsNone(patch_pkg["source"])
            self.assertEqual(patch_pkg["license"], "Apache-2.0 OR MIT")
            self.assertNotIn("checksum", policy["patched_packages"][0])
            for variant in ("registry-source", "wrong-path"):
                changed = copy.deepcopy(data)
                package = next(package for package in changed["packages"] if package["name"] == "spin_on")
                if variant == "registry-source":
                    package["source"] = PROFILE.REGISTRY
                else:
                    package["manifest_path"] = "/work/fixture/third-party/another/Cargo.toml"
                with self.assertRaises(PROFILE.Incomplete):
                    PROFILE.validate_metadata(HOST, json.dumps(changed), policy)
        for mutate in (
            lambda p: p["patched_packages"][0].update(path="third-party/another"),
            lambda p: p["patched_packages"][0].update(checksum="0" * 64),
            lambda p: p["patched_packages"][0].update(upstream_checksum="0" * 64),
            lambda p: p["patched_packages"][0].update(selected_license="Apache-2.0"),
            lambda p: p["patched_packages"][0]["modified_files_sha256"].pop("tests/stack_pin.rs"),
            lambda p: p["patched_packages"].append(copy.deepcopy(p["patched_packages"][0])),
            lambda p: p["rows"][HOST]["features"]["spin_on@0.1.1"].update(target=[[]]),
            lambda p: p["rows"][HOST]["package_scopes"]["time-core"]["features"].update({"spin_on@0.1.1": {"host": [[]]}}),
            lambda p: p["rows"][HOST]["package_scopes"]["spin_on"]["features"].update({"helper@1.0.0": {"target": [[]]}}),
        ):
            changed = copy.deepcopy(policy)
            mutate(changed)
            with self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)

    def test_patch_source_bytes_rights_and_dependency_free_lock_are_mandatory(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / "third-party/spin_on"
            shutil.copytree(ROOT / "third-party/spin_on", directory)
            record = fixture_path_patch()

            def refresh():
                files = {str(path.relative_to(directory)): VERIFY.sha256(path)
                         for path in directory.rglob("*") if path.is_file()}
                record["files_sha256"] = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
                record["modified_files_sha256"] = {name: files[name] for name in record["modified_files_sha256"]}

            refresh()
            self.assertEqual(len(VERIFY.admit_patched_source(root, record)), 12)
            manifest = directory / "Cargo.toml"
            healthy = manifest.read_text()
            manifest.write_text(healthy + '\n[dependencies]\npin-utils="0.1.1"\n')
            refresh()
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "dependency/feature/build-free"):
                VERIFY.admit_patched_source(root, record)
            manifest.write_text(healthy)
            refresh()
            license_file = directory / "LICENSE-MIT"
            license_file.write_text("Unreviewed replacement rights\n")
            refresh()
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "preserved upstream rights"):
                VERIFY.admit_patched_source(root, record)

    def test_consumer_patch_preserves_windows_gui_graph_and_separate_source_identity(self):
        policy = add_accesskit_patches(copy.deepcopy(self.policy))
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)):
            PROFILE.validate_policy(policy)
            for target in (HOST, WINDOWS):
                trees = {name: accesskit_patch_tree(output, target)
                         for name, output in fixture_package_trees(target).items()}
                trees["spin_on"] = "spin_on v0.1.1 (/work/fixture/third-party/spin_on)|\n"
                data = fixture_metadata(policy, target)
                output = accesskit_patch_tree(fixture_tree(target), target)
                projected = PROFILE.selected_metadata(target, json.dumps(data), output, policy,
                                                      package_trees=trees)
                self.assertEqual(len(projected["workspace_members"]), 7)
                consumer = [package for package in projected["packages"]
                            if package["name"] == "accesskit_consumer"]
                self.assertEqual(len(consumer), int(target == WINDOWS))
                if target == HOST:
                    continue
                self.assertIsNone(consumer[0]["source"])
                self.assertEqual(consumer[0]["license"], "(MIT OR Apache-2.0) AND GPL-3.0-only")
                for variant in ("registry-source", "wrong-path", "weaker-license"):
                    changed = copy.deepcopy(data)
                    package = next(p for p in changed["packages"] if p["name"] == "accesskit_consumer")
                    if variant == "registry-source":
                        package["source"] = PROFILE.REGISTRY
                    elif variant == "wrong-path":
                        package["manifest_path"] = "/work/fixture/third-party/another/Cargo.toml"
                    else:
                        package["license"] = "MIT OR Apache-2.0"
                    with self.subTest(variant=variant), self.assertRaises(PROFILE.Incomplete):
                        PROFILE.validate_metadata(target, json.dumps(changed), policy)
                consumer_line = "|       `-- accesskit_consumer v0.38.0 (/work/fixture/third-party/accesskit_consumer)|\n"
                for kind in ("build", "dev"):
                    changed = output.replace(consumer_line, "|       [" + kind + "-dependencies]\n" + consumer_line)
                    with self.subTest(kind=kind), self.assertRaises(PROFILE.Incomplete):
                        PROFILE.validate_tree(target, changed, policy)

    def test_consumer_patch_provenance_cohort_and_role_changes_fail_closed(self):
        policy = add_accesskit_patches(copy.deepcopy(self.policy))
        PROFILE.validate_policy_shape(policy)
        for mutate in (
            lambda p: p["patched_packages"][2].update(path="third-party/another"),
            lambda p: p["patched_packages"][2].update(checksum="0" * 64),
            lambda p: p["patched_packages"][2].update(upstream_checksum="0" * 64),
            lambda p: p["patched_packages"][2].update(selected_license="MIT"),
            lambda p: p["patched_packages"][2]["modified_files_sha256"].pop("src/text/nevrotick_word_navigation_tests.rs"),
            lambda p: p["patched_packages"].reverse(),
            lambda p: p["patched_packages"].pop(1),
            lambda p: p["patched_packages"].append(copy.deepcopy(p["patched_packages"][2])),
            lambda p: p["rows"][HOST]["metadata_features"].update({"accesskit_consumer@0.38.0": ["unreviewed"]}),
            lambda p: p["rows"][HOST]["features"].update({"accesskit_consumer@0.38.0": {"target": [[]]}}),
            lambda p: p["rows"][WINDOWS]["features"]["accesskit_consumer@0.38.0"].update(host=[[]]),
            lambda p: p["rows"][WINDOWS]["package_scopes"]["nevrotick-ui"]["features"].pop("accesskit_consumer@0.38.0"),
            lambda p: p["rows"][WINDOWS]["package_scopes"]["time-core"]["features"].update(
                {"accesskit_consumer@0.38.0": {"target": [[]]}}),
        ):
            changed = copy.deepcopy(policy)
            mutate(changed)
            with self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)

    def test_consumer_patch_lock_retains_only_its_original_normal_edges(self):
        patches = [fixture_path_patch(), *fixture_accesskit_patches()]
        packages = [
            {"name": "spin_on", "version": "0.1.1"},
            {"name": "accesskit_windows", "version": "0.34.0",
             "dependencies": ["accesskit", "accesskit_consumer", "hashbrown 0.16.1", "static_assertions", "windows", "windows-core"]},
            {"name": "accesskit_consumer", "version": "0.38.0", "dependencies": ["accesskit", "hashbrown 0.16.1"]},
        ]
        VERIFY.validate_patched_lock(packages, patches)
        for mutation in ({"checksum": "0" * 64}, {"source": PROFILE.REGISTRY},
                         {"dependencies": ["accesskit", "hashbrown 0.16.1", "unreviewed"]}):
            changed = copy.deepcopy(packages)
            changed[2].update(mutation)
            with self.subTest(mutation=mutation), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_patched_lock(changed, patches)

    def test_consumer_source_preserves_upstream_manifest_notices_and_unmodified_bytes(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / "third-party/accesskit_consumer"
            shutil.copytree(ROOT / "third-party/accesskit_consumer", directory)
            record = fixture_accesskit_patches()[1]

            def refresh():
                files = {path.relative_to(directory).as_posix(): VERIFY.sha256(path)
                         for path in directory.rglob("*") if path.is_file()}
                record["files_sha256"] = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
                record["modified_files_sha256"] = {name: files[name] for name in record["modified_files_sha256"]}

            refresh()
            self.assertEqual(len(VERIFY.admit_patched_source(root, record)), 18)
            for name in ("LICENSE-MIT", "LICENSE-APACHE", "LICENSE-GPL-3.0-only", "Cargo.toml.orig",
                         "Cargo.toml.upstream", "Cargo.lock", "src/tree.rs"):
                path = directory / name
                original = path.read_bytes()
                path.write_bytes(original + b"\nchanged\n")
                refresh()
                with self.subTest(name=name), self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.admit_patched_source(root, record)
                path.write_bytes(original)
            manifest = directory / "Cargo.toml"
            healthy = manifest.read_text()
            self.assertIn('version = "0.24.1"', healthy)
            manifest.write_text(healthy.replace('version = "0.24.1"', 'version = "0.24.2"'))
            refresh()
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "manifest may only change"):
                VERIFY.admit_patched_source(root, record)
            manifest.write_text(healthy)
            refresh()
            VERIFY.admit_patched_source(root, record)
            (directory / "src/text.rs").write_text("// unreviewed replacement\n")
            with self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.admit_patched_source(root, record)

    def test_consumer_grant_and_commands_remain_bounded_to_existing_gui_selection(self):
        policy = add_accesskit_patches(copy.deepcopy(self.policy))
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)):
            for target in (HOST, WINDOWS):
                generated = tomllib.loads(VERIFY.product_deny((ROOT / "deny.toml").read_text(), policy, target))
                self.assertEqual(generated["licenses"]["allow"], ["GPL-2.0-only"])
                consumers = [item for item in generated["licenses"]["exceptions"]
                             if item["crate"] == "accesskit_consumer@0.38.0"]
                self.assertEqual(consumers, [{"crate": "accesskit_consumer@0.38.0", "allow": ["MIT", "GPL-3.0-only"]}]
                                 if target == WINDOWS else [])
            for lane in ("portable", "windows", "dependencies"):
                self.assertEqual(VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on", "accesskit_windows")),
                                 VERIFY.commands(lane, schema_version=3,
                                                 patched_packages=("spin_on", "accesskit_windows", "accesskit_consumer")))

    def test_archive_bypasses_bind_exact_paths_and_bytes_without_shortcuts(self):
        policy = copy.deepcopy(self.policy)
        policy["archive_file_bypasses"] = [{"package": "helper@2.0.0", "path": "tests/fixture.dll",
                                            "sha256": "3" * 64, "reason": "Inert exact test-only fixture."}]
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)):
            generated = tomllib.loads(VERIFY.product_deny((ROOT / "deny.toml").read_text(), policy, HOST))
            bypasses = generated["bans"]["build"]["bypass"]
            self.assertEqual(bypasses, [{"crate": "helper@2.0.0", "allow": [{"path": "tests/fixture.dll", "checksum": "3" * 64}]}])
            self.assertEqual(set(bypasses[0]), {"crate", "allow"})
            self.assertEqual(generated["bans"]["build"]["executables"], "deny")
            self.assertEqual(generated["bans"]["build"]["interpreted"], "deny")
        for mutation in ({"path": "../escape"}, {"path": "tests/*.dll"}, {"path": "/tmp/file"},
                         {"sha256": ""}, {"package": "helper"}, {"allow_globs": ["**"]},
                         {"build_script": "3" * 64}, {"required_features": []}):
            changed = copy.deepcopy(policy)
            changed["archive_file_bypasses"][0].update(mutation)
            with self.subTest(mutation=mutation), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)

    def test_patch_checks_follow_all_nine_source_scopes(self):
        for lane in ("portable", "windows", "dependencies"):
            rows = VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on",))
            self.assertEqual(sum(label.startswith("dependency-tree-") for label, _ in rows),
                             18 if lane == "dependencies" else 9)
            if lane == "portable":
                self.assertEqual(rows[10][0], "patch-compiler")
                for name in ("product-tests:spin_on", "doctests:spin_on"):
                    command = dict(rows)[name]
                    self.assertIn("--manifest-path", command)
                    self.assertIn("--frozen", command)

    def test_frozen_tree_distinguishes_target_build_and_proc_macro_contexts(self):
        result = PROFILE.validate_tree(HOST, fixture_tree(), self.policy)
        self.assertEqual(result["role_feature_contexts"], 13)
        self.assertNotIn("skia-bindings@1.0.0", result["selected_packages"])

    def test_package_scope_requires_its_own_roots_and_feature_contexts(self):
        ui_tree = fixture_tree().split("ntp-wire v0.0.0")[0]
        PROFILE.validate_tree(HOST, ui_tree, self.policy, scope="nevrotick-ui")
        with self.assertRaises(PROFILE.Incomplete):
            PROFILE.validate_tree(HOST, fixture_tree(), self.policy, scope="nevrotick-ui")
        with self.assertRaisesRegex(PROFILE.Incomplete, "software profile"):
            changed = copy.deepcopy(self.policy)
            changed["rows"][HOST]["features"]["slint@1.18.1"] = {"target": [RUNTIME_FEATURES]}
            PROFILE.validate_policy_shape(changed)

    def test_exchange_scope_retains_dev_core_closure_and_no_local_feature_definitions(self):
        scope = self.policy["rows"][HOST]["package_scopes"]["ntp-exchange"]
        self.assertEqual(set(scope["local_features"]),
                         {name + "@0.0.0" for name in ("ntp-exchange", "ntp-wire", "time-domain", "time-core")})
        self.assertTrue(all(value == {"target": [[]]} for value in scope["local_features"].values()))
        command = next(command for label, command in VERIFY.commands("portable", schema_version=3)
                       if label == "dependency-tree-ntp-exchange-" + HOST)
        self.assertIn("normal,build,dev", command)
        workspace = VERIFY.commands("portable", schema_version=3)[1][1]
        self.assertIn("normal,build", workspace)
        output = ("ntp-exchange v0.0.0 (/work/fixture/crates/ntp-exchange)|\n"
                  "|-- ntp-wire v0.0.0 (/work/fixture/crates/ntp-wire)|\n"
                  "`-- time-domain v0.0.0 (/work/fixture/crates/time-domain)|\n"
                  "[dev-dependencies]\n"
                  "`-- time-core v0.0.0 (/work/fixture/crates/time-core)|\n"
                  "    `-- time-domain v0.0.0 (/work/fixture/crates/time-domain)|\n")
        PROFILE.validate_tree(HOST, output, self.policy, scope="ntp-exchange")
        for changed in (output.replace("[dev-dependencies]", "[build-dependencies]"),
                        output + "[dev-dependencies]\n",
                        output.replace("[dev-dependencies]", "[dev-dependencies]\n[dev-dependencies]")):
            with self.subTest(changed=changed), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_tree(HOST, changed, self.policy, scope="ntp-exchange")

    def test_engine_scope_requires_exact_time_domain_closure_on_both_targets(self):
        expected = {name + "@0.0.0": {"target": [[]]}
                    for name in ("nevrotick-engine", "ntp-wire", "time-domain")}
        for target in (HOST, WINDOWS):
            with self.subTest(target=target):
                scope = self.policy["rows"][target]["package_scopes"]["nevrotick-engine"]
                self.assertEqual(scope["local_features"], expected)
                PROFILE.validate_policy_shape(self.policy)
                PROFILE.validate_tree(target, fixture_package_trees(target)["nevrotick-engine"],
                                      self.policy, scope="nevrotick-engine")
            for variant in ("missing", "extra", "version", "feature", "host", "boolean-feature"):
                changed = copy.deepcopy(self.policy)
                local = changed["rows"][target]["package_scopes"]["nevrotick-engine"]["local_features"]
                if variant == "missing":
                    del local["time-domain@0.0.0"]
                elif variant == "extra":
                    local["time-core@0.0.0"] = {"target": [[]]}
                elif variant == "version":
                    local["time-domain@0.0.1"] = local.pop("time-domain@0.0.0")
                elif variant == "feature":
                    local["time-domain@0.0.0"] = {"target": [["default"]]}
                elif variant == "host":
                    local["time-domain@0.0.0"] = {"host": [[]]}
                else:
                    local["time-domain@0.0.0"] = {"target": [[True]]}
                with self.subTest(target=target, variant=variant), self.assertRaisesRegex(
                        PROFILE.Incomplete, "Package scope"):
                    PROFILE.validate_policy_shape(changed)

    def test_engine_normal_time_domain_edge_survives_projection_on_both_targets(self):
        expected_edge = {"parent": "nevrotick-engine@0.0.0", "parent_features": [],
                         "parent_role": "target", "child": "time-domain@0.0.0", "child_features": [],
                         "child_role": "target", "kind": "normal"}
        for target in (HOST, WINDOWS):
            with self.subTest(target=target):
                trees = fixture_package_trees(target)
                observed = PROFILE.validate_tree(target, trees["nevrotick-engine"], self.policy,
                                                 scope="nevrotick-engine", include_edges=True)
                self.assertIn(expected_edge, observed["dependency_edges"])
                data = fixture_metadata(self.policy, target)
                projected = PROFILE.selected_metadata(target, json.dumps(data), fixture_tree(target), self.policy,
                                                      package_trees=trees)
                engine = next(package for package in projected["packages"] if package["name"] == "nevrotick-engine")
                domain = next(package for package in projected["packages"] if package["name"] == "time-domain")
                declaration = next(dep for dep in engine["dependencies"] if dep["name"] == "time-domain")
                self.assertEqual(declaration, {"name": "time-domain", "rename": None, "req": "=0.0.0",
                                               "kind": None, "target": None, "optional": False,
                                               "uses_default_features": False, "features": []})
                node = next(node for node in projected["resolve"]["nodes"] if node["id"] == engine["id"])
                self.assertIn({"name": "time_domain", "pkg": domain["id"],
                               "dep_kinds": [{"kind": None, "target": None}]}, node["deps"])

    def test_engine_tree_refuses_domain_source_role_and_feature_drift_on_both_targets(self):
        domain = "`-- time-domain v0.0.0 (/work/fixture/crates/time-domain)|\n"
        for target in (HOST, WINDOWS):
            healthy = fixture_package_trees(target)["nevrotick-engine"]
            self.assertEqual(healthy.count(domain), 1)
            PROFILE.validate_tree(target, healthy, self.policy, scope="nevrotick-engine")
            variants = {
                "missing": healthy.replace(domain, ""),
                "build": healthy.replace(domain, "[build-dependencies]\n" + domain),
                "version": healthy.replace(domain, domain.replace("v0.0.0", "v0.0.1")),
                "source": healthy.replace(domain, domain.replace("crates/time-domain", "crates/time-core")),
                "extra": healthy + "`-- time-core v0.0.0 (/work/fixture/crates/time-core)|\n",
                "feature": healthy.replace(domain, domain.replace("|\n", "|default\n")),
            }
            for variant, output in variants.items():
                with self.subTest(target=target, variant=variant), self.assertRaises(PROFILE.Incomplete):
                    PROFILE.validate_tree(target, output, self.policy, scope="nevrotick-engine")

    def test_engine_projection_refuses_domain_dev_and_build_kinds_on_both_targets(self):
        domain = "`-- time-domain v0.0.0 (/work/fixture/crates/time-domain)|\n"
        for target in (HOST, WINDOWS):
            healthy = fixture_metadata(self.policy, target)
            trees = fixture_package_trees(target)
            PROFILE.selected_metadata(target, json.dumps(healthy), fixture_tree(target), self.policy,
                                      package_trees=trees)
            for kind in ("dev", "build"):
                for variant in ("tree", "declaration", "resolved"):
                    data, changed_trees = copy.deepcopy(healthy), copy.deepcopy(trees)
                    engine = next(package for package in data["packages"] if package["name"] == "nevrotick-engine")
                    declaration = next(dep for dep in engine["dependencies"] if dep["name"] == "time-domain")
                    node = next(node for node in data["resolve"]["nodes"] if node["id"] == engine["id"])
                    edge = next(edge for edge in node["deps"] if edge["name"] == "time_domain")
                    if variant == "tree":
                        changed_trees["nevrotick-engine"] = trees["nevrotick-engine"].replace(
                            domain, "[" + kind + "-dependencies]\n" + domain)
                    elif variant == "declaration":
                        declaration["kind"] = kind
                    else:
                        edge["dep_kinds"] = [{"kind": kind, "target": None}]
                    with self.subTest(target=target, kind=kind, variant=variant), self.assertRaisesRegex(
                            PROFILE.Incomplete, "exact Cargo declaration|lost an observed dependency edge|target/host features"):
                        PROFILE.selected_metadata(target, json.dumps(data), fixture_tree(target), self.policy,
                                                  package_trees=changed_trees)

    def test_selected_tree_rejects_metadata_promotion_role_feature_and_source_drift(self):
        healthy = fixture_tree()
        variants = (
            healthy.replace("|-- helper v2.0.0|", "|-- skia-bindings v1.0.0|metadata-gpu-only"),
            healthy.replace("[build-dependencies]\n", ""),
            healthy.replace("|narrow", "|default"),
            healthy.replace(" (proc-macro)", ""),
            healthy.replace("|-- helper v1.0.0|narrow", "|-- helper v1.0.0 (https://unreviewed.invalid)|narrow"),
            healthy.replace("`-- helper v1.0.0|narrow", "`-- helper v1.0.0|narrow (*)"),
            healthy + "[build-dependencies]\n",
            healthy.replace("|-- helper v2.0.0|", "|-- helper v2.0.0|x,x"),
            healthy.replace("ntp-wire v0.0.0", "unreviewed v0.0.0"),
            healthy.replace("|-- helper v1.0.0", "|   |   |-- helper v1.0.0"),
        )
        for index, output in enumerate(variants):
            with self.subTest(index=index), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_tree(HOST, output, self.policy)

    def test_same_role_dedupe_template_is_accepted(self):
        output = fixture_tree().replace("ntp-wire v0.0.0 (/work/fixture/crates/ntp-wire)|\n",
                                       "ntp-wire v0.0.0 (/work/fixture/crates/ntp-wire)|\n`-- helper v1.0.0|narrow (*)\n")
        PROFILE.validate_tree(HOST, output, self.policy)

    def test_conflicting_full_templates_for_one_context_are_rejected(self):
        output = fixture_tree().replace("ntp-wire v0.0.0 (/work/fixture/crates/ntp-wire)|\n",
            "ntp-wire v0.0.0 (/work/fixture/crates/ntp-wire)|\n`-- helper v1.0.0|narrow\n    `-- helper v2.0.0|\n")
        with self.assertRaisesRegex(PROFILE.Incomplete, "Conflicting full"):
            PROFILE.validate_tree(HOST, output, self.policy)

    def test_metadata_overapproximation_is_checked_and_pruned_after_selected_tree(self):
        data = fixture_metadata(self.policy)
        result = PROFILE.validate_metadata(HOST, json.dumps(data), self.policy)
        self.assertEqual(result["inert_metadata_extras"], ["skia-bindings@1.0.0"])
        projected = PROFILE.selected_metadata(HOST, json.dumps(data), fixture_tree(), self.policy,
                                              package_trees=fixture_package_trees())
        self.assertNotIn("skia-bindings", {package["name"] for package in projected["packages"]})
        ids = {package["id"] for package in projected["packages"]}
        self.assertEqual(ids, {node["id"] for node in projected["resolve"]["nodes"]})
        for node in projected["resolve"]["nodes"]:
            self.assertTrue(set(node["dependencies"]) <= ids)
            self.assertTrue({dep["pkg"] for dep in node["deps"]} <= ids)
        exchange = next(package for package in projected["packages"] if package["name"] == "ntp-exchange")
        self.assertEqual(next(dep for dep in exchange["dependencies"] if dep["name"] == "time-core")["kind"], "dev")

    def test_projection_drops_inactive_weak_edges_between_two_selected_packages(self):
        """Model the actual potential_utf/serde_core alloc panic without running Cargo."""
        policy = copy.deepcopy(self.policy)
        for row in policy["rows"].values():
            row["features"]["helper@2.0.0"] = {"target": [["std"]]}
            row["package_scopes"]["nevrotick-ui"]["features"]["helper@2.0.0"] = {"target": [["std"]]}
            row["metadata_features"]["helper@2.0.0"] = ["alloc", "std"]
        data = fixture_metadata(policy)
        parent = next(package for package in data["packages"] if package["name"] == "helper" and package["version"] == "1.0.0")
        child = next(package for package in data["packages"] if package["name"] == "helper" and package["version"] == "2.0.0")
        parent["features"]["narrow"] = ["serde_core?/alloc"]
        parent["dependencies"].append({"name": "helper", "rename": "serde_core", "req": "=2.0.0",
                                       "kind": None, "target": None, "optional": True,
                                       "uses_default_features": True, "features": ["alloc"]})
        node = next(node for node in data["resolve"]["nodes"] if node["id"] == parent["id"])
        node["dependencies"] = [child["id"]]
        node["deps"] = [{"name": "serde_core", "pkg": child["id"], "dep_kinds": [{"kind": None, "target": None}]}]
        tree = fixture_tree().replace("helper v2.0.0|", "helper v2.0.0|std")
        trees = {scope: output.replace("helper v2.0.0|", "helper v2.0.0|std")
                 for scope, output in fixture_package_trees().items()}
        with patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(policy)):
            projected = PROFILE.selected_metadata(HOST, json.dumps(data), tree, policy, package_trees=trees)
        selected_parent = next(package for package in projected["packages"] if package["id"] == parent["id"])
        selected_child = next(node for node in projected["resolve"]["nodes"] if node["id"] == child["id"])
        self.assertEqual(selected_parent["features"]["narrow"], [])
        self.assertEqual(selected_parent["dependencies"], [])
        self.assertEqual(next(node for node in projected["resolve"]["nodes"] if node["id"] == parent["id"])["deps"], [])
        self.assertEqual(selected_child["features"], ["std"])
        self.assertTrue(any(dep["pkg"] == child["id"] for node in projected["resolve"]["nodes"] for dep in node["deps"]))
        self.assertEqual({package["id"] for package in projected["packages"]},
                         {package["id"] for package in data["packages"] if package["name"] != "skia-bindings"})
        for package in projected["packages"]:
            source = next(raw for raw in data["packages"] if raw["id"] == package["id"])
            self.assertEqual({key: value for key, value in package.items() if key not in {"dependencies", "features"}},
                             {key: value for key, value in source.items() if key not in {"dependencies", "features"}})

    def test_projection_requires_every_package_tree_and_matching_cargo_edge(self):
        data = fixture_metadata(self.policy)
        with self.assertRaisesRegex(PROFILE.Incomplete, "all admitted"):
            PROFILE.selected_metadata(HOST, json.dumps(data), fixture_tree(), self.policy)
        data["packages"][-len(MEMBERS)]["dependencies"] = []
        with self.assertRaisesRegex(PROFILE.Incomplete, "exact Cargo declaration"):
            PROFILE.selected_metadata(HOST, json.dumps(data), fixture_tree(), self.policy,
                                      package_trees=fixture_package_trees())

    def test_metadata_source_license_execution_target_feature_and_duplicate_drift_reject(self):
        healthy = fixture_metadata(self.policy)
        for variant in ("source", "license", "build", "macro", "links", "features", "duplicate", "edge", "workspace"):
            data = copy.deepcopy(healthy)
            if variant == "source": data["packages"][0]["source"] = "git+https://unreviewed.invalid"
            elif variant == "license": data["packages"][0]["license"] = "GPL-3.0-only"
            elif variant == "build": data["packages"][0]["targets"] = [{"kind": ["lib"]}]
            elif variant == "macro": data["packages"][0]["targets"].append({"kind": ["proc-macro"]})
            elif variant == "links": data["packages"][0]["links"] = "unreviewed-native"
            elif variant == "features": data["resolve"]["nodes"][0]["features"] = ["full"]
            elif variant == "duplicate": data["packages"][0] = copy.deepcopy(data["packages"][1])
            elif variant == "edge": data["resolve"]["nodes"][0]["dependencies"] = ["missing"]
            else: data["workspace_members"].pop()
            with self.subTest(variant=variant), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_metadata(HOST, json.dumps(data), self.policy)

    def test_metadata_json_duplicates_and_output_bounds_reject(self):
        for output in ('{"packages":[],"packages":[]}', '{"number":NaN}', '[]'):
            with self.subTest(output=output), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_metadata(HOST, output, self.policy)
        with patch.object(PROFILE, "MAX_TREE_BYTES", 32):
            with self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_tree(HOST, fixture_tree(), self.policy)

    def test_commands_preflight_every_schema3_lane_before_package_execution(self):
        for lane in ("portable", "windows", "dependencies"):
            rows = VERIFY.commands(lane, schema_version=3)
            self.assertTrue(rows[0][0].startswith("dependency-metadata-"))
            self.assertTrue(rows[1][0].startswith("dependency-tree-"))
            self.assertIn("--offline", rows[0][1])
            self.assertIn("normal,build", rows[1][1])
            if lane == "dependencies":
                self.assertEqual(len(rows), 20)
                for label, command in rows:
                    if label in ("dependency-" + HOST, "dependency-" + WINDOWS):
                        self.assertIn("--metadata-path", command)
            else:
                self.assertIn(rows[9][0], ("format", "windows-compiler"))
            self.assertEqual(sum(label.startswith("dependency-tree-") for label, _ in rows),
                             16 if lane == "dependencies" else 8)

    def test_deny_grants_and_duplicate_exceptions_are_exact_and_keep_other_bans(self):
        base = (ROOT / "deny.toml").read_text()
        for target in (HOST, WINDOWS):
            generated = tomllib.loads(VERIFY.product_deny(base, self.policy, target))
            original = tomllib.loads(base)
            self.assertEqual(generated["licenses"]["allow"], ["GPL-2.0-only"])
            exceptions = {item["crate"]: item["allow"] for item in generated["licenses"]["exceptions"]}
            self.assertEqual(set(exceptions), set(self.policy["rows"][target]["features"]) | {"nevrotick-ui@0.0.0"})
            self.assertEqual(exceptions["helper@1.0.0"], ["MIT"])
            self.assertEqual(exceptions["helper@2.0.0"], ["Apache-2.0"])
            self.assertNotIn("skia-bindings@1.0.0", exceptions)
            self.assertEqual([item["crate"] for item in generated["bans"]["skip"]], ["helper@1.0.0", "helper@2.0.0"])
            generated["licenses"]["exceptions"] = []
            generated["bans"]["skip"] = []
            generated["bans"]["build"]["allow-build-scripts"] = []
            self.assertEqual(generated, original)

    def test_preflight_failure_prevents_any_compiler_or_build_command(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            project = base / "project"
            names = ("Cargo.lock", "deny.toml", ".config/nextest.toml", *VERIFY.CHECKER_FILES,
                     *VERIFY.UI_TEXT_CHECKER_FILES, *VERIFY.UI_NATIVE_INPUT_CHECKER_FILES)
            for name in names:
                destination = project / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, destination)
            source = {name: VERIFY.sha256(project / name) for name in names}
            for name in ("vendor", "toolchain", "scratch"):
                (base / name).mkdir()
            args = argparse.Namespace(lane="portable", vendor_dir=base / "vendor", toolchain_dir=base / "toolchain",
                                      scratch_root=base / "scratch", nextest=None, deny=None, output_dir=None)
            for failed_scope in ("workspace", "nevrotick-ui"):
                observed = []

                def run(command, **kwargs):
                    observed.append((command, kwargs))
                    if "metadata" in command:
                        output = json.dumps(fixture_metadata(self.policy))
                    elif "--workspace" in command and failed_scope != "workspace":
                        output = fixture_tree()
                    else:
                        output = "malformed tree"
                    return {"exit_code": 0, "failure": None, "output": output}

                with self.subTest(failed_scope=failed_scope), \
                        patch.object(VERIFY, "admit_product", return_value=(self.policy, source)), \
                        patch.object(VERIFY, "admit_vendor", return_value={}), \
                        patch.object(VERIFY, "admit_tools", return_value={}), \
                        patch.object(VERIFY, "sandbox", side_effect=lambda _t, _w, _s, _v, command, *_args: command), \
                        patch.object(VERIFY, "boundary_control"), \
                        patch.object(VERIFY, "text_contract_commands", return_value=[]), \
                        patch.object(VERIFY, "native_input_control_commands", return_value=[]), \
                        patch.object(VERIFY.ENV, "run", side_effect=run):
                    with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "tree"):
                        VERIFY.execute(project, args, {})
                self.assertEqual(len(observed), 2 if failed_scope == "workspace" else 3)
                self.assertIn("metadata", observed[0][0])
                self.assertEqual(observed[0][1]["max_output"], 8 * 1024 * 1024)
                self.assertIn("tree", observed[1][0])
                self.assertNotIn("max_output", observed[1][1])
                if failed_scope != "workspace":
                    self.assertIn("--package", observed[-1][0])
                    self.assertIn("nevrotick-ui", observed[-1][0])


if __name__ == "__main__":
    unittest.main()
