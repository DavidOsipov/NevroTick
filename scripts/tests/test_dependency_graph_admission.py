"""Independent changed-graph age/custody controls; no registry or package code."""
from __future__ import annotations

import copy
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import tomllib
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("graph_cooldown_control", ROOT / "scripts/dependency_cooldown.py")
AGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AGE)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def policy():
    return json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())


def changed_policy():
    """Replace one registry identity beside every real unchanged transitive."""
    value = policy()
    original = next(p for p in value["packages"] if p["name"] == "ryu")
    old = {key: original[key] for key in ("name", "version", "checksum")}
    old["source"] = AGE.REGISTRY
    original["version"], original["checksum"] = "99.0.0", "1" * 64
    new = dict(old, version="99.0.0", checksum="1" * 64)
    records = AGE.policy_records(value)
    value["lockfile_sha256"] = "2" * 64
    record = {"scope": "product", "baseline_lockfile_sha256": AGE.BASELINE_LOCK,
        "baseline_admission_sha256": AGE.canonical(AGE.baseline_admission()), "retained_exception_decisions": [],
        "acquisition_filesystem": {"bytes": 536870912, "inodes": 16384, "device": 1},
        "baseline_registry_sha256": AGE.BASELINE_REGISTRY, "candidate_lockfile_sha256": "2" * 64,
        "candidate_graph_sha256": "3" * 64, "candidate_registry_sha256": AGE.canonical(records),
        "operation": "precise", "package": "ryu", "version": "99.0.0",
        "started_utc": "2026-11-13T12:00:00Z", "finished_utc": "2026-11-13T12:00:03Z",
        "cutoff_utc": "2026-10-30T12:00:00Z", "cargo_sha256": "4" * 64,
        "configuration_sha256": "5" * 64, "manifests_sha256": value["manifests_sha256"].copy(),
        "command": ["/toolchain/bin/cargo", "update", "--package", "ryu", "--precise", "99.0.0"],
        "exit_code": 0, "failure": None, "added": [new], "removed": [old],
        "publication": [{"package": new, "published_utc": "2026-10-29T12:00:00Z",
            "source_url": "https://index.crates.io/3/r/ryu", "index_record_sha256": "6" * 64,
            "index_record": {"name": "ryu", "vers": "99.0.0", "cksum": "1" * 64,
                             "pubtime": "2026-10-29T12:00:00Z", "deps": []}}],
        "exception": None, "metadata_inventory_sha256": "7" * 64, "result_sha256": "8" * 64,
        "generated_lockfile_sha256": "2" * 64, "proposed_lockfile_sha256": None}
    record["publication"][0]["index_record_sha256"] = AGE.canonical(record["publication"][0]["index_record"])
    value["cooldown_admission"].update(status="resolved", resolution=record)
    return value


class GraphAgeControls(unittest.TestCase):
    def test_exact_current_accepted_baseline_remains_accepted_without_age_reclassification(self):
        value = policy()
        AGE.validate_policy_admission(value)
        AGE.validate_root_admission(ROOT, value)
        self.assertEqual(value["cooldown_admission"], AGE.baseline_admission())

    def test_only_age_subrecord_was_added_to_accepted_registry_graph(self):
        value = policy()
        self.assertEqual(AGE.canonical(AGE.policy_records(value)),
            "622bd793fc19f6edea04a104463028739ec80a842da17a04981ff5b7d15e68b7")
        self.assertEqual(sha((ROOT / "Cargo.lock").read_bytes()), AGE.BASELINE_LOCK)

    def test_relabelled_new_version_or_lock_is_not_grandfathered(self):
        for field in ("version", "checksum", "lock"):
            value = policy()
            if field == "lock":
                value["lockfile_sha256"] = "a" * 64
            else:
                value["packages"][0][field] = "99.0.0" if field == "version" else "a" * 64
            with self.subTest(field=field), self.assertRaises(AGE.Incomplete):
                AGE.validate_policy_admission(value)

    def test_complete_old_enough_resolved_delta_has_a_healthy_neighbor(self):
        AGE.validate_policy_admission(changed_policy())

    def test_young_unknown_or_wrong_time_source_version_is_rejected(self):
        for change in ("young", "unknown", "wrong-source", "wrong-package", "duplicate", "missing"):
            value = changed_policy()
            record = value["cooldown_admission"]["resolution"]
            item = record["publication"][0]
            if change == "young": item["published_utc"] = "2026-10-31T12:00:00Z"
            if change == "unknown": item["published_utc"] = None
            if change in ("young", "unknown"):
                item["index_record"]["pubtime"] = item["published_utc"]
                item["index_record_sha256"] = AGE.canonical(item["index_record"])
            if change == "wrong-source": item["source_url"] = "https://unreviewed.invalid/ryu"
            if change == "wrong-package": item["package"] = dict(item["package"], version="98.0.0")
            if change == "duplicate": record["publication"].append(copy.deepcopy(item))
            if change == "missing": record["publication"] = []
            with self.subTest(change=change), self.assertRaises(AGE.Incomplete):
                AGE.validate_policy_admission(value)

    def test_omitted_added_removed_or_transitive_identity_fails_reconstruction(self):
        for change in ("added", "removed", "transitive"):
            value = changed_policy()
            record = value["cooldown_admission"]["resolution"]
            if change in ("added", "removed"): record[change] = []
            else:
                record["removed"] += [dict(name="forged-transitive", version="1.0.0", source=AGE.REGISTRY, checksum="a" * 64)]
                record["removed"].sort(key=lambda p: tuple(p[k] for k in ("name", "version", "source", "checksum")))
            with self.subTest(change=change), self.assertRaises(AGE.Incomplete):
                AGE.validate_policy_admission(value)

    def test_missing_resolution_failed_child_or_wrong_cutoff_is_not_evidence(self):
        for change in ("absent", "failure", "cutoff", "interval", "override", "different-generated-lock", "wrong-manifest"):
            value = changed_policy()
            record = value["cooldown_admission"]["resolution"]
            if change == "absent": value["cooldown_admission"]["resolution"] = None
            if change == "failure": record["exit_code"] = 101
            if change == "cutoff": record["cutoff_utc"] = "2026-10-31T12:00:00Z"
            if change == "interval": record["finished_utc"] = "2026-11-13T12:06:00Z"
            if change == "override": record["command"] += ["--config", 'resolver.incompatible-publish-age="allow"']
            if change == "different-generated-lock": record["generated_lockfile_sha256"] = "f" * 64
            if change == "wrong-manifest": record["manifests_sha256"]["Cargo.toml"] = "f" * 64
            with self.subTest(change=change), self.assertRaises(AGE.Incomplete):
                AGE.validate_policy_admission(value)

    def test_full_lock_graph_hash_preserves_edge_only_import_differences(self):
        first = b'version=4\n[[package]]\nname="local"\nversion="0.0.0"\ndependencies=["first"]\n'
        second = first.replace(b'"first"', b'"second"')
        self.assertEqual(AGE.registry_records(first), AGE.registry_records(second))
        self.assertNotEqual(AGE.canonical(AGE.lock_document(first)), AGE.canonical(AGE.lock_document(second)))
        AGE.require_reproduced_lock(first, first)
        with self.assertRaisesRegex(AGE.Incomplete, "graph/edges"):
            AGE.require_reproduced_lock(first, second)

    def test_actual_manifest_change_cannot_publish_unchanged_baseline(self):
        value = policy()
        value["manifests_sha256"]["Cargo.toml"] = "f" * 64
        AGE.validate_policy_admission(value)
        with self.assertRaisesRegex(AGE.Incomplete, "manifest"):
            AGE.validate_root_admission(ROOT, value)

    def test_actual_full_lock_rewiring_invalidates_record_even_with_same_registry_versions(self):
        value = policy()
        record = changed_policy()["cooldown_admission"]["resolution"]
        original = (ROOT / "Cargo.lock").read_bytes()
        changed = original.replace(b' "garde",', b' "ryu",', 1)
        self.assertNotEqual(original, changed)
        self.assertEqual(AGE.registry_records(original), AGE.registry_records(changed))
        record.update(candidate_lockfile_sha256=AGE.BASELINE_LOCK, generated_lockfile_sha256=AGE.BASELINE_LOCK,
            candidate_registry_sha256=AGE.canonical(AGE.policy_records(value)),
            candidate_graph_sha256=AGE.canonical(AGE.lock_document(original)), added=[], removed=[], publication=[],
            cargo_sha256=json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]["linux_binary_sha256"]["cargo"],
            configuration_sha256=sha((ROOT / ".cargo/config.toml").read_bytes()), manifests_sha256=value["manifests_sha256"])
        value["cooldown_admission"].update(status="resolved", resolution=record)
        AGE.validate_root_admission(ROOT, value)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in (*value["manifests_sha256"], ".cargo/config.toml", "policy/tool-manifest.json"):
                destination = root / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes((ROOT / name).read_bytes())
            (root / "Cargo.lock").write_bytes(changed)
            value["lockfile_sha256"] = record["candidate_lockfile_sha256"] = record["generated_lockfile_sha256"] = sha(changed)
            AGE.validate_policy_admission(value)
            with self.assertRaisesRegex(AGE.Incomplete, "full lock graph"):
                AGE.validate_root_admission(root, value)

    def test_missing_or_forged_retained_raw_publication_record_is_not_accepted(self):
        for change in ("absent", "timestamp", "checksum", "row-hash"):
            value = changed_policy()
            item = value["cooldown_admission"]["resolution"]["publication"][0]
            if change == "absent": item.pop("index_record")
            if change == "timestamp": item["index_record"]["pubtime"] = "2026-01-01T00:00:00Z"
            if change == "checksum": item["index_record"]["cksum"] = "f" * 64
            if change == "row-hash": item["index_record_sha256"] = "f" * 64
            with self.subTest(change=change), self.assertRaises(AGE.Incomplete):
                AGE.validate_policy_admission(value)

    def test_unsupported_fractional_publication_time_is_not_rounded_into_eligibility(self):
        value = changed_policy()
        item = value["cooldown_admission"]["resolution"]["publication"][0]
        item["published_utc"] = item["index_record"]["pubtime"] = "2026-10-30T12:00:00.0001Z"
        item["index_record_sha256"] = AGE.canonical(item["index_record"])
        with self.assertRaises(AGE.Incomplete): AGE.validate_policy_admission(value)

    def test_forbidden_nonregistry_or_duplicate_lock_identity(self):
        for suffix in ('source="git+https://example.invalid/repo"\nchecksum="'+'a'*64+'"\n', ''):
            text = 'version=4\n[[package]]\nname="one"\nversion="1.0.0"\n'+suffix
            if not suffix: text += '[[package]]\nname="one"\nversion="1.0.0"\n'
            with self.subTest(suffix=suffix), self.assertRaises(AGE.Incomplete): AGE.lock_document(text.encode())


class ChildBoundaryControls(unittest.TestCase):
    def test_oversized_inputs_fail_before_body_read_or_json_decode(self):
        custody = AGE._input_custody()
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "oversized"
            for maximum in (AGE.MAX_LOCK_BYTES, AGE.MAX_MANIFEST_BYTES, AGE.MAX_CONFIGURATION_BYTES, AGE.MAX_TOOL_BYTES):
                with path.open("wb") as stream: stream.truncate(maximum + 1)
                with self.subTest(maximum=maximum), patch.object(custody.os, "read", side_effect=AssertionError("Body read preceded size admission")):
                    with self.assertRaises(AGE.Incomplete): AGE.read_input(path, maximum)
                    with self.assertRaises(AGE.Incomplete): AGE.digest_input(path, maximum)
        with patch.object(AGE.json, "loads", side_effect=AssertionError("Decoded oversized raw JSON")):
            with self.assertRaises(AGE.Incomplete): AGE._strict_json(b"x" * (AGE.MAX_RECORD_BYTES + 1))

    def test_bounded_input_rejects_redirected_ancestor_beside_healthy_regular_file(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            real = root / "real"
            real.mkdir()
            (real / "data").write_bytes(b"bounded")
            (root / "alias").symlink_to(real, target_is_directory=True)
            self.assertEqual(AGE.read_input(real / "data", 7), b"bounded")
            with self.assertRaises(AGE.Incomplete): AGE.read_input(root / "alias/data", 7)

    def test_fixed_precise_workspace_initial_commands_reject_arbitrary_overrides(self):
        self.assertEqual(AGE.resolution_command("/toolchain/bin/cargo", "precise", "ryu", "1.0.23"),
            ["/toolchain/bin/cargo", "update", "--package", "ryu", "--precise", "1.0.23"])
        self.assertEqual(AGE.resolution_command("/toolchain/bin/cargo", "workspace"), ["/toolchain/bin/cargo", "update", "--workspace"])
        self.assertEqual(AGE.resolution_command("/toolchain/bin/cargo", "initial"), ["/toolchain/bin/cargo", "generate-lockfile"])
        self.assertEqual(AGE.resolution_command("/toolchain/bin/cargo", "precise", "syn@2.0.119", "2.0.120"),
            ["/toolchain/bin/cargo", "update", "--package", "syn@2.0.119", "--precise", "2.0.120"])
        for args in (("--config",), ("workspace", "extra", "1.0.0"), ("precise", "ryu", "1 --config=allow")):
            with self.subTest(args=args), self.assertRaises(AGE.Incomplete): AGE.resolution_command("/toolchain/bin/cargo", *args)

    def test_caller_cannot_claim_urgent_authority_and_normal_env_never_inherits_allow(self):
        with patch.dict(os.environ, {"CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE": "allow", "SECRET_TOKEN": "must-not-copy"}):
            ordinary = AGE.resolution_environment("/toolchain", "/work/cargo-home", "/work/tmp")
        self.assertNotIn("CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE", ordinary)
        self.assertNotIn("SECRET_TOKEN", ordinary)
        with self.assertRaises(AGE.Incomplete):
            AGE.resolution_environment("/toolchain", "/work/cargo-home", "/work/tmp", {"approved_by": "David"})

    def test_one_concrete_exception_binds_entire_resulting_transitive_delta(self):
        value = changed_policy()
        record = value["cooldown_admission"]["resolution"]
        record["publication"][0]["published_utc"] = "2026-10-31T12:00:00Z"
        record["publication"][0]["index_record"]["pubtime"] = "2026-10-31T12:00:00Z"
        record["publication"][0]["index_record_sha256"] = AGE.canonical(record["publication"][0]["index_record"])
        exception = {"decision_id": "FIXTURE-OD04-ONE", "decision_ref": "fixture://explicit-only",
            "decision_sha256": "d" * 64, "requested_package": "ryu", "requested_version": "99.0.0",
            "reason": "Inert accepted decision control", "primary_fix_evidence": "https://fixture.invalid/fix",
            "targets": ["x86_64-unknown-linux-gnu"], "features": [],
            "delta_sha256": AGE.canonical({"added": record["added"], "removed": record["removed"]})}
        record["exception"] = exception
        with patch.dict(AGE.APPROVED_EXCEPTIONS, {exception["decision_id"]: AGE.canonical(exception)}):
            AGE.validate_policy_admission(value)
            child = AGE.resolution_environment("/toolchain", "/work/cargo-home", "/work/tmp", exception)
            self.assertEqual(child["CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE"], "allow")
            self.assertNotIn("CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE", AGE.resolution_environment("/toolchain", "/work/next", "/work/tmp"))
            record["added"][0] = dict(record["added"][0], checksum="e" * 64)
            with self.assertRaises(AGE.Incomplete): AGE.validate_policy_admission(value)
        self.assertEqual(AGE.APPROVED_EXCEPTIONS, {})

    def test_second_update_uses_current_accepted_checkpoint_without_rechecking_old_exception(self):
        first = changed_policy()
        first_age = first["cooldown_admission"]
        first_record = first_age["resolution"]
        item = first_record["publication"][0]
        item["published_utc"] = item["index_record"]["pubtime"] = "2026-10-31T12:00:00Z"
        item["index_record_sha256"] = AGE.canonical(item["index_record"])
        exception = {"decision_id": "OD04-RETAINED-YOUNG-RYU", "decision_ref": "fixture://accepted-prior",
            "decision_sha256": "d" * 64, "requested_package": "ryu", "requested_version": "99.0.0",
            "reason": "Inert prior accepted young version", "primary_fix_evidence": "https://fixture.invalid/fix",
            "targets": ["x86_64-unknown-linux-gnu"], "features": [],
            "delta_sha256": AGE.canonical({"added": first_record["added"], "removed": first_record["removed"]})}
        first_record["exception"] = exception
        with patch.dict(AGE.APPROVED_EXCEPTIONS, {exception["decision_id"]: AGE.canonical(exception)}):
            AGE.validate_policy_admission(first)
        first_id = AGE.canonical(first_age)
        second = copy.deepcopy(first)
        package = next(item for item in second["packages"] if item["name"] == "itoa")
        old = {key: package[key] for key in ("name", "version", "checksum")}
        old["source"] = AGE.REGISTRY
        package["version"], package["checksum"] = "99.0.0", "b" * 64
        new = dict(old, version="99.0.0", checksum="b" * 64)
        record = second["cooldown_admission"]["resolution"]
        second["lockfile_sha256"] = "9" * 64
        record.update(baseline_admission_sha256=first_id, baseline_lockfile_sha256=first["lockfile_sha256"],
            baseline_registry_sha256=AGE.canonical(AGE.policy_records(first)), candidate_lockfile_sha256="9" * 64,
            generated_lockfile_sha256="9" * 64, candidate_registry_sha256=AGE.canonical(AGE.policy_records(second)),
            package="itoa", command=AGE.resolution_command("/toolchain/bin/cargo", "precise", "itoa", "99.0.0"),
            added=[new], removed=[old], retained_exception_decisions=["OD04-RETAINED-YOUNG-RYU"], exception=None)
        raw = {"name": "itoa", "vers": "99.0.0", "cksum": "b" * 64, "pubtime": "2026-10-29T12:00:00Z"}
        record["publication"] = [{"package": new, "published_utc": raw["pubtime"], "source_url": "https://index.crates.io/it/oa/itoa",
                                  "index_record": raw, "index_record_sha256": AGE.canonical(raw)}]
        second["cooldown_admission"].update(baseline_lockfile_sha256=record["baseline_lockfile_sha256"],
            baseline_registry_sha256=record["baseline_registry_sha256"])
        with self.assertRaisesRegex(AGE.Incomplete, "checkpoint"):
            AGE.validate_policy_admission(second)
        checkpoint = {"lockfile_sha256": first["lockfile_sha256"], "registry_sha256": AGE.canonical(AGE.policy_records(first)),
                      "exception_decisions": ["OD04-RETAINED-YOUNG-RYU"]}
        with patch.dict(AGE.APPROVED_BASELINES, {first_id: checkpoint}):
            AGE.validate_policy_admission(second)
            record["retained_exception_decisions"] = []
            with self.assertRaisesRegex(AGE.Incomplete, "prior exception"):
                AGE.validate_policy_admission(second)
        self.assertEqual(AGE.APPROVED_BASELINES, {})

    def test_symlinked_output_ancestor_refuses_before_any_publication(self):
        snapshot = AGE._load("output_ancestor_control", ROOT / "scripts/advisory_snapshot.py")
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            (base / "alias").symlink_to(ROOT, target_is_directory=True)
            output = base / "alias" / ("must-not-publish-" + base.name)
            self.assertFalse(output.exists())
            with self.assertRaises((AGE.Incomplete, ValueError)):
                AGE._output_parent(output, snapshot)
            self.assertFalse(output.exists())

    def test_private_output_parent_identity_and_payloads_are_checked(self):
        snapshot = AGE._load("output_publication_control", ROOT / "scripts/advisory_snapshot.py")
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            output = base / "new"
            before = AGE._output_parent(output, snapshot)
            AGE._publish(output, {"Cargo.lock": b'inert', "cooldown-resolution.json": b'{}', "resolver-result.json": b'{}'}, snapshot, before)
            self.assertEqual((output / "cooldown-resolution.json").read_bytes(), b'{}')
            self.assertEqual((output / "cooldown-resolution.json").stat().st_mode & 0o777, 0o600)
            with self.assertRaises(AGE.Incomplete): AGE._output_parent(output, snapshot)

    def test_direct_publisher_cannot_escape_or_expand_its_bounded_inventory(self):
        snapshot = AGE._load("output_inventory_control", ROOT / "scripts/advisory_snapshot.py")
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "new"
            before = AGE._output_parent(output, snapshot)
            for payloads in ({"../escaped": b'bad'}, {"Cargo.lock": b'incomplete'},
                             {"Cargo.lock": b'inert', "cooldown-resolution.json": b'x'*(AGE.MAX_RECORD_BYTES+1), "resolver-result.json": b'{}'}):
                with self.assertRaises(AGE.Incomplete): AGE._publish(output, payloads, snapshot, before)
                self.assertFalse(output.exists())
            self.assertFalse((output.parent / "escaped").exists())


class ResolutionProducerControls(unittest.TestCase):
    """Exercise real owned source/export plumbing with only Cargo/quota seams."""

    def exercise(self, fault=None):
        snapshot = AGE._load("producer_snapshot_control", ROOT / "scripts/advisory_snapshot.py")
        original_load = AGE._load
        tool = json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]
        toolchain = Path.home() / ".rustup/toolchains" / (tool["channel"] + "-" + tool["host"])
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            candidate, quota, output = base / "candidate", base / "quota", base / "result"
            candidate.mkdir(mode=0o700)
            quota.mkdir(mode=0o700)
            (candidate / ".cargo").mkdir()
            (candidate / "src").mkdir()
            config = (ROOT / ".cargo/config.toml").read_bytes()
            (candidate / ".cargo/config.toml").write_bytes(config)
            (candidate / "Cargo.toml").write_text('[package]\nname="local"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nselected-leaf="1"\n')
            (candidate / "src/lib.rs").write_text("pub fn inert() {}\n")
            lock = ('version=4\n[[package]]\nname="local"\nversion="0.0.0"\ndependencies=["selected-leaf"]\n'
                    '[[package]]\nname="selected-leaf"\nversion="1.0.0"\nsource="'+AGE.REGISTRY+'"\nchecksum="'+'c'*64+'"\n').encode()
            proposal = base / "proposal.lock"
            if fault == "import-edge": proposal.write_bytes(lock.replace(b'dependencies=["selected-leaf"]', b'dependencies=["other"]'))
            if fault == "config": (candidate / ".cargo/config.toml").write_bytes(config.replace(b'14 days', b'0 days'))
            if fault == "tool": toolchain = base / "unadmitted-toolchain"
            if fault == "tool":
                (toolchain / "bin").mkdir(parents=True)
                (toolchain / "bin/cargo").write_text("unadmitted bytes")
            parent_environment = {}
            if fault == "age-env": parent_environment["CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE"] = "0 days"
            if fault == "home-config":
                home = base / "inherited-home"
                (home / ".cargo").mkdir(parents=True)
                (home / ".cargo/config.toml").write_text('[registry]\nglobal-min-publish-age="0 days"\n')
                parent_environment["HOME"] = str(home)
            args = SimpleNamespace(candidate_root=candidate, baseline_lock=None, proposed_lock=proposal if fault == "import-edge" else None,
                quota_root=quota, output_dir=output, toolchain_dir=toolchain, operation="initial", scope="standalone",
                package=None, version=None, exception=None)
            entered = []
            def cargo_metadata(argv, *, cwd, environment):
                entered.append(argv)
                self.assertEqual(argv[-2:], ["/toolchain/bin/cargo", "generate-lockfile"])
                self.assertIn("--unshare-all", argv)
                self.assertIn("--cap-drop", argv)
                self.assertIn("--clearenv", argv)
                work = Path(argv[argv.index("--bind") + 1])
                project = work / "project"
                self.assertEqual((project / "Cargo.toml").read_bytes(), (candidate / "Cargo.toml").read_bytes())
                (project / "Cargo.lock").write_bytes(lock)
                raw = {"name": "selected-leaf", "vers": "1.0.0", "cksum": "c" * 64, "deps": [],
                       "pubtime": "2020-01-01T00:00:00Z" if fault != "young" else "2099-01-01T00:00:00Z"}
                cache = work / "cargo-home/registry/index/inert/.cache/se/le/selected-leaf"
                cache.parent.mkdir(parents=True)
                cache.write_bytes(b'\x03\x02\0etag:inert\0' + b'1.0.0\0' + json.dumps(raw).encode() + b'\0')
                if fault == "source-drift": (candidate / "src/lib.rs").write_text("changed source")
                if fault == "context-drift":
                    home = base / "changed-cargo-home"
                    home.mkdir()
                    (home / "config.toml").write_text('[registry]\nglobal-min-publish-age="0 days"\n')
                    os.environ["CARGO_HOME"] = str(home)
                return {"command": argv, "exit_code": 0, "failure": None, "output": "Controlled metadata-only resolution"}
            def loader(name, path):
                return snapshot if name == "cooldown_snapshot" else original_load(name, path)
            with patch.dict(os.environ, parent_environment, clear=True), patch.object(AGE, "_load", side_effect=loader), \
                    patch.object(snapshot, "quota_root", return_value={"bytes": 536870912, "inodes": 16384, "device": quota.stat().st_dev}), \
                    patch.object(snapshot, "run", side_effect=cargo_metadata):
                if fault:
                    with self.assertRaises((AGE.Incomplete, RuntimeError, OSError)):
                        AGE.resolve(args)
                    self.assertFalse(output.exists(), "Refused graph must not publish a lock/age receipt")
                    if fault in ("config", "tool", "age-env", "home-config"): self.assertEqual(entered, [])
                else:
                    result = AGE.resolve(args)
                    retained = json.loads((output / "cooldown-resolution.json").read_text())
                    self.assertEqual(retained, result)
                    self.assertEqual((output / "Cargo.lock").read_bytes(), lock)
                    self.assertEqual(result["publication"][0]["index_record"]["pubtime"], "2020-01-01T00:00:00Z")
                    self.assertEqual(result["publication"][0]["index_record_sha256"], AGE.canonical(result["publication"][0]["index_record"]))
                    self.assertEqual(result["generated_lockfile_sha256"], result["candidate_lockfile_sha256"])
                    self.assertEqual(result["scope"], "standalone")
                    self.assertEqual((candidate / ".cargo/config.toml").read_bytes(), config)
                    self.assertEqual((candidate / "src/lib.rs").read_text(), "pub fn inert() {}\n")
                    self.assertEqual(len(entered), 1)
            self.assertEqual(list(quota.iterdir()), [], "Owned resolution scratch must be removed on every exit")

    def test_healthy_producer_exports_exact_lock_raw_rows_and_receipt_without_touching_source(self):
        self.exercise()

    def test_young_metadata_is_refused_before_publication(self):
        self.exercise("young")

    def test_imported_edge_only_change_is_refused_before_publication(self):
        self.exercise("import-edge")

    def test_changed_source_context_is_refused_before_publication(self):
        self.exercise("source-drift")

    def test_wrong_tool_and_weakened_config_fail_before_child_execution(self):
        for fault in ("tool", "config", "age-env", "home-config"):
            with self.subTest(fault=fault): self.exercise(fault)

    def test_changed_parent_cargo_context_refuses_publication(self):
        self.exercise("context-drift")


if __name__ == "__main__":
    unittest.main()
