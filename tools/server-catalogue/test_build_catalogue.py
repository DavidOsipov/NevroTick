"""Bounded synthetic contract checks; run with the documented isolated host route."""

from __future__ import annotations

import contextlib
import copy
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("catalogue_under_test", HERE / "build_catalogue.py")
assert SPEC is not None and SPEC.loader is not None
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)
ZERO_KEY = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
ONE_KEY = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE="


def dump(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=True, sort_keys=True, separators=(",", ":")) + "\n").encode("ascii")


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def rough_row(name: str = "Alpha", host: str = "rough.invalid:5319") -> dict:
    return {"name": name, "version": 1, "publicKeyType": "ed25519", "publicKey": ZERO_KEY,
            "addresses": [{"protocol": "udp", "address": host}]}


class Case(unittest.TestCase):
    def setUp(self) -> None:
        self.owned = tempfile.TemporaryDirectory(prefix="catalogue-test-")
        self.addCleanup(self.owned.cleanup)
        self.directory = Path(self.owned.name)
        self.serial = 0

    def fresh(self, suffix: str = "json") -> Path:
        self.serial += 1
        return self.directory / f"case-{self.serial}.{suffix}"

    def fixture(self, name: str) -> bytes:
        return (HERE / "fixtures" / name).read_bytes()

    def manifest(self, entries: list[tuple[str, str, bytes]]) -> Path:
        sources = []
        for source_id, adapter, data in entries:
            path = self.fresh("input")
            path.write_bytes(data)
            sources.append({"id": source_id, "adapter": adapter, "file": path.name,
                            "sha256": sha(data), "bytes": len(data),
                            "origin": {"uri": "https://evidence.invalid/" + source_id,
                                       "revision": None, "git_blob": None,
                                       "license_uri": None, "related": []}})
        path = self.fresh()
        path.write_bytes(dump({"schema": "nevrotick-r-input-v1", "sources": sources}))
        return path

    def invoke(self, args: list[str]) -> tuple[int, str, str]:
        stdout, stderr = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            status = C.main(args)
        return status, stdout.getvalue(), stderr.getvalue()

    def collect(self, entries: list[tuple[str, str, bytes]]) -> tuple[int, dict, Path]:
        manifest = self.manifest(entries)
        output = self.fresh()
        status, _, _ = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertIn(status, (0, 3))
        return status, json.loads(output.read_bytes()), output

    def one_rough(self, rows: list[dict] | None = None) -> tuple[int, dict, Path]:
        return self.collect([("rough", C.ROUGH, dump({"servers": rows if rows is not None else [rough_row()]}))])

    def difference(self, old: Path, new: Path) -> tuple[int, dict | None, str]:
        output = self.fresh()
        status, _, error = self.invoke(["diff", "--before", str(old), "--before-sha256", sha(old.read_bytes()),
                                       "--after", str(new), "--after-sha256", sha(new.read_bytes()),
                                       "--output", str(output)])
        return status, json.loads(output.read_bytes()) if output.exists() else None, error

    def refuses(self, function, *args, code: str | None = None) -> None:
        with self.assertRaises(C.Refusal) as captured:
            function(*args)
        if code is not None:
            self.assertEqual(captured.exception.code, code)


class ParsingTests(Case):
    def test_exact_json_types_and_duplicate_members(self) -> None:
        self.assertEqual(C.strict_json(b'{"version":1}'), {"version": 1})
        for value in (b'{"x":1,"x":1}', b'{"x":1,"x":2}', b'{"x":{"a":1,"a":2}}'):
            with self.subTest(value=value):
                self.refuses(C.strict_json, value, code="json-duplicate")
        for value in (b"1.0", b"1e0", b"NaN", b"Infinity"):
            with self.subTest(value=value):
                self.refuses(C.strict_json, value, code="json-float")
        for value in (b"{} {}", b'{"x":', b"[1,]", b'\xef\xbb\xbf{}', b'"\\ud800"', b'"\\u202e"'):
            with self.subTest(value=value):
                self.refuses(C.strict_json, value)

    def test_canonical_public_key_bytes(self) -> None:
        self.assertEqual(C.key_material("ed25519", ZERO_KEY)["hex"], "00" * 32)
        self.assertEqual(C.key_material("ed25519", ONE_KEY)["hex"], "01" * 32)
        for bad in ("AA==", ZERO_KEY[:-1], ZERO_KEY[:-2] + "B=", ZERO_KEY + "=", " " + ZERO_KEY,
                    "_" + ZERO_KEY[1:]):
            with self.subTest(spelling=bad):
                self.refuses(C.key_material, "ed25519", bad, code="key")

    def test_literal_and_dns_endpoints(self) -> None:
        dns = C.endpoint("MiXeD.Invalid.", "nts", "tcp")
        self.assertEqual(dns["host"], {"kind": "dns", "value": "mixed.invalid."})
        self.assertEqual((dns["explicit_port"], dns["effective_port"], dns["port_basis"]),
                         (None, 4460, "adapter-default:rfc8915-section4"))
        ipv4 = C.endpoint("192.0.2.33:0", "roughtime", "udp")
        self.assertEqual(ipv4["host"], {"kind": "ipv4", "value": "192.0.2.33"})
        self.assertEqual((ipv4["explicit_port"], ipv4["effective_port"], ipv4["port_state"]),
                         (0, 0, "zero-unusable"))
        ipv6 = C.endpoint("[2001:0DB8:0:0:0:0:0:33]:5319", "roughtime", "tcp")
        self.assertEqual(ipv6["host"], {"kind": "ipv6", "value": "2001:db8::33"})
        self.assertEqual(ipv6["effective_port"], 5319)
        self.assertEqual(C.endpoint("0.amazon.pool.ntp.org", "ntp", "udp")["alias_kind"],
                         "dynamic-pool-alias")
        self.assertEqual(C.endpoint("pool-like.invalid", "ntp", "udp")["alias_kind"],
                         "not-classified")

    def test_rejects_ambiguous_endpoint_neighbors(self) -> None:
        for token in ("2001:db8::33:5319", "[2001:db8::33", "2001:db8::33]", "[192.0.2.1]:5319",
                      "[fe80::1%zone]:5319", "999.1.1.1:5319", "010.0.0.1:5319", "127.1:5319",
                      "0x7f.0.0.1:5319", "2130706433:5319", "https://good.invalid:5319",
                      "good.invalid/path:5319", "x@good.invalid:5319", "good.invalid..:5319",
                      "-bad.invalid:5319", "bad-.invalid:5319", "café.invalid:5319",
                      "good.invalid:05319", "good.invalid:65536", "good.invalid:-1", "good.invalid:",
                      "good.invalid", " good.invalid:5319", "good.invalid:5319\x1b"):
            with self.subTest(token=token):
                self.refuses(C.endpoint, token, "roughtime", "udp")

    def test_dns_and_port_boundaries(self) -> None:
        longest = ".".join(["a" * 63] * 3 + ["a" * 61])
        self.assertEqual(len(C.endpoint(longest, "ntp", "udp")["host"]["value"]), 254)
        self.refuses(C.endpoint, longest + "a", "ntp", "udp", code="endpoint")
        self.refuses(C.endpoint, "a" * 64 + ".invalid", "ntp", "udp", code="endpoint")
        self.assertEqual(C.endpoint("time.invalid:65535", "ntp", "udp")["effective_port"], 65535)
        self.refuses(C.endpoint, "time.invalid:65536", "ntp", "udp", code="port")

    def test_supported_and_unsupported_profile_accounting(self) -> None:
        status, value, _ = self.collect([("rough", C.ROUGH, self.fixture("roughtime.json"))])
        self.assertEqual(status, 0)
        rows = value["sources"][0]["records"]
        self.assertEqual(len(rows), 3)
        self.assertEqual([row["meaning"]["disposition"] for row in rows],
                         ["candidate-unreviewed", "unresolved-profile", "unsupported-algorithm"])
        self.assertEqual([row["meaning"]["profile"]["supported_profile"] for row in rows], [None] * 3)
        self.assertEqual([len(row["meaning"]["endpoints"]) for row in rows], [2, 1, 1])
        self.assertEqual(rows[0]["meaning"]["key"]["hex"], "00" * 32)
        future = rough_row()
        future["version"] = 2
        self.assertEqual(self.one_rough([future])[1]["sources"][0]["records"][0]["meaning"]["disposition"],
                         "unsupported-profile")

    def test_wrong_profile_types_do_not_coerce(self) -> None:
        for version in (True, False, 1.0, "1", "01", "+1", "1.0", "draft-any", None, 0, 65536):
            with self.subTest(version=version):
                raw = rough_row()
                raw["version"] = version
                status, value, _ = self.one_rough([raw])
                self.assertEqual(status, 3)
                self.assertEqual(value["sources"][0]["status"], "failed")
                self.assertEqual(value["record_count"], 0)

    def test_roughtime_closed_shapes_and_keys(self) -> None:
        valid = {"servers": [rough_row()]}
        bodies = [[], {}, {"servers": []}, {"servers": {}}, {**valid, "reports": []}]
        for mutation in ({"publicKeyType": "rsa"}, {"publicKey": "AA=="}, {"addresses": []},
                         {"addresses": [{"protocol": "udp", "address": "rough.invalid:5319", "active": True}]},
                         {"addresses": [{"protocol": True, "address": "rough.invalid:5319"}]},
                         {"admitted": True}):
            bodies.append({"servers": [{**rough_row(), **mutation}]})
        for body in bodies:
            with self.subTest(body=body):
                status, value, _ = self.collect([("rough", C.ROUGH, dump(body))])
                self.assertEqual(status, 3)
                self.assertEqual(value["record_count"], 0)


class CollectionTests(Case):
    def test_three_families_and_independent_literal_expectations(self) -> None:
        entries = [("nts", C.NTS, self.fixture("nts.md")), ("ntp", C.NTP, self.fixture("ntp.md")),
                   ("rough", C.ROUGH, self.fixture("roughtime.json"))]
        status, value, _ = self.collect(entries)
        self.assertEqual((status, value["status"], value["record_count"]), (0, "complete-candidate", 9))
        self.assertEqual(value["authority"], "unapproved-publisher-evidence")
        sources = {source["id"]: source for source in value["sources"]}
        nts = sources["nts"]["records"]
        self.assertEqual(nts[0]["locator"], {"line": 6, "table": "unmarked"})
        first = nts[0]["meaning"]
        self.assertEqual(first["endpoints"][0]["host"]["value"], "nts.example.invalid.")
        self.assertEqual(first["information_uri"], "https://info.invalid/docs")
        self.assertEqual(first["claims"]["notes"], "Distro use only")
        self.assertEqual(first["claims"]["location"], "Distro")
        self.assertEqual(nts[1]["meaning"]["claims"]["location"], "All")
        self.assertEqual(nts[2]["meaning"]["claims"]["virtualization"], "source-claimed-virtualized")
        ntp = sources["ntp"]["records"]
        self.assertEqual(ntp[1]["meaning"]["claims"]["as"], "AS1, AS2")
        self.assertEqual(ntp[2]["raw"]["stratum"], "Unknown")
        for source in sources.values():
            self.assertEqual(source["artifact"]["sha256"], source["observed"]["sha256"])
            for item in source["records"]:
                self.assertEqual(item["meaning"]["time_behavior"], "unqualified")
                self.assertEqual(item["meaning"]["use_permission"], "unknown")
                self.assertEqual(item["meaning"]["independence"], "unknown")
                self.assertEqual(item["meaning"]["geography"], "unknown")

    def test_complete_markdown_shape_is_required(self) -> None:
        good = self.fixture("nts.md")
        variants = [good.replace(b"## Star History", b"## Changed End"),
                    good + b"\n## The List\n", good.replace(b"|Hostname|Stratum|", b"|Host|Stratum|"),
                    good.replace(b"|Distro|Example Owner|", b"|Distro|extra|Example Owner|"),
                    good.replace(C.VM_MARKER.encode(), b"Different VM marker"),
                    good.replace(C.VM_MARKER.encode(), b""),
                    good.replace(b"|vm.invalid|2|Unknown|VM Owner||", b""),
                    good.replace(b"|Hostname|Stratum|Location|Owner|Notes|", b"||", 1),
                    good.replace(b"\n## Star History", b"\n||\n## Star History"),
                    good.replace(b"\n||\n", b"\n||\n||\n", 1),
                    good.replace(b"[Nts.Example.Invalid.]", b"[https://nts.example.invalid/path]")]
        for body in variants:
            with self.subTest(body=sha(body)):
                status, value, _ = self.collect([("bad", C.NTS, body), ("good", C.NTP, self.fixture("ntp.md"))])
                self.assertEqual((status, value["status"], value["record_count"]), (3, "incomplete-candidate", 3))
                self.assertEqual(value["sources"][0]["records"], [])
                self.assertEqual(value["sources"][1]["status"], "complete")
        self.assertEqual(self.collect([("bad", C.NTS, self.fixture("ntp.md"))])[0], 3)

    def test_hash_length_blob_and_missing_source_failures_preserve_neighbor(self) -> None:
        for mutation in ("digest", "length", "blob", "missing"):
            with self.subTest(mutation=mutation):
                manifest = self.manifest([("bad", C.NTS, self.fixture("nts.md")),
                                          ("good", C.NTP, self.fixture("ntp.md"))])
                value = json.loads(manifest.read_bytes())
                source = value["sources"][0]
                if mutation == "digest":
                    source["sha256"] = "0" * 64
                elif mutation == "length":
                    source["bytes"] += 1
                elif mutation == "blob":
                    source["origin"]["git_blob"] = "0" * 40
                else:
                    (self.directory / source["file"]).unlink()
                manifest.write_bytes(dump(value))
                output = self.fresh()
                self.assertEqual(self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])[0], 3)
                result = json.loads(output.read_bytes())
                self.assertEqual(result["record_count"], 3)
                self.assertEqual(result["sources"][0]["status"], "failed")
                self.assertEqual(result["sources"][1]["status"], "complete")

    def test_deterministic_bytes_source_sort_and_raw_identity(self) -> None:
        manifest = self.manifest([("z-nts", C.NTS, self.fixture("nts.md")),
                                  ("a-ntp", C.NTP, self.fixture("ntp.md"))])
        left, right = self.fresh(), self.fresh()
        for output in (left, right):
            self.assertEqual(self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])[0], 0)
        self.assertEqual(left.read_bytes(), right.read_bytes())
        value = json.loads(left.read_bytes())
        self.assertEqual([item["id"] for item in value["sources"]], ["a-ntp", "z-nts"])
        source, item = value["sources"][0], value["sources"][0]["records"][0]
        independent = {"source_id": "a-ntp", "artifact_sha256": source["artifact"]["sha256"],
                       "locator": item["locator"], "raw": item["raw"]}
        self.assertEqual(item["claim_id"], sha(dump(independent)))
        self.assertNotIn(str(self.directory).encode(), left.read_bytes())

    def test_conflicting_and_duplicate_claims_keep_exact_bindings(self) -> None:
        left, right = rough_row(), rough_row("Beta")
        right["publicKey"] = ONE_KEY
        status, value, _ = self.one_rough([left, right])
        self.assertEqual(status, 0)
        self.assertEqual(value["review_groups"][0]["kind"], "differing-claims")
        self.assertEqual(len(value["review_groups"][0]["claim_ids"]), 2)
        rows = value["sources"][0]["records"]
        self.assertEqual([len(row["meaning"]["endpoints"]) for row in rows], [1, 1])
        self.assertEqual([row["meaning"]["key"]["hex"] for row in rows], ["00" * 32, "01" * 32])
        duplicate = self.one_rough([left, copy.deepcopy(left)])[1]
        self.assertEqual(duplicate["record_count"], 2)
        self.assertEqual(duplicate["review_groups"][0]["kind"], "equivalent-claims")
        self.assertEqual(len(set(duplicate["review_groups"][0]["claim_ids"])), 2)

    def test_inert_markup_and_redacted_diagnostics(self) -> None:
        body = self.fixture("nts.md").replace(b"Example Owner", b"<script>alert(1)</script> **owner**")
        status, value, path = self.collect([("nts", C.NTS, body)])
        self.assertEqual(status, 0)
        self.assertIn("<script>", value["sources"][0]["records"][0]["raw"]["owner"])
        self.assertNotIn(b"<script>", path.read_bytes())
        status, stdout, stderr = self.invoke(["collect", "--bogus", "\x1b<script>secret</script>"])
        self.assertEqual((status, stdout, stderr), (2, "", "refused: arguments\n"))

    def test_manifest_and_symlink_boundaries(self) -> None:
        for bad in ("../outside", "/outside", "https://outside.invalid/input"):
            manifest = self.manifest([("good", C.NTS, self.fixture("nts.md"))])
            value = json.loads(manifest.read_bytes())
            value["sources"][0]["file"] = bad
            manifest.write_bytes(dump(value))
            output = self.fresh()
            self.assertEqual(self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])[0], 2)
            self.assertFalse(output.exists())
        regular, link = self.fresh(), self.fresh()
        regular.write_bytes(b"healthy")
        link.symlink_to(regular.name)
        self.refuses(C.read_regular, str(link), 100)
        self.assertEqual(regular.read_bytes(), b"healthy")


class DiffTests(Case):
    def test_identical_and_reordered_claims_have_no_semantic_change(self) -> None:
        rows = [rough_row(), rough_row("Beta", "beta.invalid:2002")]
        _, _, old = self.one_rough(rows)
        _, _, new = self.one_rough(list(reversed(rows)))
        for right, provenance in ((old, False), (new, True)):
            status, result, _ = self.difference(old, right)
            self.assertEqual(status, 0)
            item = result["sources"][0]
            self.assertEqual((item["added"], item["absent"], item["changed"], item["unchanged_groups"]),
                             ([], [], [], 2))
            self.assertEqual(item["provenance_changed"], provenance)
            self.assertEqual(result["before"]["sha256"], sha(old.read_bytes()))

    def test_key_profile_endpoint_and_absent_differences(self) -> None:
        rows = [rough_row(), rough_row("Beta", "beta.invalid:2002")]
        _, _, old = self.one_rough(rows)
        for field, value, expected in (("publicKey", ONE_KEY, "key"),
                                       ("version", "IETF-Roughtime", "profile"),
                                       ("addresses", [{"protocol": "tcp", "address": "other.invalid:2002"}], "endpoints")):
            changed = copy.deepcopy(rows)
            changed[0][field] = value
            _, _, new = self.one_rough(changed)
            status, result, _ = self.difference(old, new)
            self.assertEqual(status, 0)
            changes = result["sources"][0]["changed"]
            self.assertEqual(len(changes), 1)
            self.assertEqual(changes[0]["join"], {"family": "roughtime", "anchor": "Alpha"})
            self.assertIn(expected, changes[0]["fields"])
        _, _, reduced = self.one_rough(rows[:1])
        status, result, _ = self.difference(old, reduced)
        self.assertEqual(status, 0)
        self.assertEqual(result["sources"][0]["absent"][0]["join"]["anchor"], "Beta")
        self.assertNotIn("retired", dump(result).decode())

    def test_rebinding_under_a_duplicate_name_is_visible(self) -> None:
        rows = [rough_row(), rough_row(host="other.invalid:5319")]
        rows[1]["publicKey"] = ONE_KEY
        _, _, old = self.one_rough(rows)
        rows[0]["publicKey"], rows[1]["publicKey"] = ONE_KEY, ZERO_KEY
        _, _, new = self.one_rough(rows)
        status, result, _ = self.difference(old, new)
        self.assertEqual(status, 0)
        self.assertEqual(result["sources"][0]["changed"][0]["fields"], ["binding-scope"])

    def test_missing_failed_and_changed_adapter_sources_block_row_comparison(self) -> None:
        nts = self.fixture("nts.md")
        ntp = self.fixture("ntp.md")
        _, _, old = self.collect([("stable", C.NTS, nts), ("other", C.NTP, ntp)])
        cases = [[("other", C.NTP, ntp)], [("stable", C.NTS, b"truncated"), ("other", C.NTP, ntp)],
                 [("stable", C.NTP, ntp), ("other", C.NTP, ntp)]]
        for entries in cases:
            _, _, new = self.collect(entries)
            status, result, _ = self.difference(old, new)
            self.assertEqual(status, 3)
            item = next(value for value in result["sources"] if value["id"] == "stable")
            self.assertEqual(item["comparison"], "blocked")
            self.assertEqual((item["added"], item["absent"], item["changed"]), ([], [], []))

    def test_table_reorder_and_metadata_change(self) -> None:
        body = self.fixture("ntp.md")
        first = b"|google-like.invalid|AS15169|1|Google NTP|Example||"
        second = b"|pool-like.invalid|AS1, AS2|2|Vendor group|Nominal Vendor||"
        reordered = body.replace(first, b"PLACEHOLDER").replace(second, first).replace(b"PLACEHOLDER", second)
        _, _, old = self.collect([("ntp", C.NTP, body)])
        _, _, new = self.collect([("ntp", C.NTP, reordered)])
        result = self.difference(old, new)[1]["sources"][0]
        self.assertEqual((result["added"], result["absent"], result["changed"], result["unchanged_groups"]),
                         ([], [], [], 3))
        _, _, changed = self.collect([("ntp", C.NTP, body.replace(b"|Example||", b"|Example|Use only by arrangement|"))])
        change = self.difference(old, changed)[1]["sources"][0]["changed"][0]
        self.assertEqual(change["fields"], ["claims"])
        self.assertEqual(change["after"][0]["time_behavior"], "unqualified")

    def test_forged_snapshot_cannot_bypass_revalidation(self) -> None:
        _, value, old = self.one_rough()
        forged_values = []
        for location, key, supplied in (("root", "admitted", True), ("root", "record_count", True),
                                        ("root", "authority", "approved-release"),
                                        ("meaning", "use_permission", "approved"),
                                        ("meaning", "independence", "independent"),
                                        ("meaning", "time_behavior", "qualified"),
                                        ("meaning", "key", {"algorithm": "ed25519", "hex": "ff" * 32}),
                                        ("source", "active", True)):
            forged = copy.deepcopy(value)
            target = (forged if location == "root" else forged["sources"][0] if location == "source"
                      else forged["sources"][0]["records"][0]["meaning"])
            target[key] = supplied
            forged_values.append(forged)
        broken = copy.deepcopy(value)
        broken["sources"][0]["records"][0]["raw"]["publicKey"] = "AA=="
        forged_values.append(broken)
        for forged in forged_values:
            path = self.fresh()
            path.write_bytes(dump(forged))
            status, result, error = self.difference(old, path)
            self.assertEqual(status, 2)
            self.assertIsNone(result)
            self.assertTrue(error.startswith("refused: "))
        self.assertEqual(self.difference(old, old)[0], 0)

    def test_snapshot_digest_and_canonical_bytes_are_required(self) -> None:
        _, value, old = self.one_rough()
        output = self.fresh()
        status, _, error = self.invoke(["diff", "--before", str(old), "--before-sha256", "0" * 64,
                                       "--after", str(old), "--after-sha256", sha(old.read_bytes()),
                                       "--output", str(output)])
        self.assertEqual((status, error), (2, "refused: snapshot-digest\n"))
        self.assertFalse(output.exists())
        pretty = self.fresh()
        pretty.write_text(json.dumps(value, indent=2), encoding="utf-8")
        self.assertEqual(self.difference(old, pretty)[0], 2)


class BoundaryTests(Case):
    def test_json_depth_member_item_string_and_integer_boundaries(self) -> None:
        self.assertEqual(C.strict_json(b"[" * 12 + b"0" + b"]" * 12)[0][0][0][0][0][0][0][0][0][0][0][0], 0)
        self.refuses(C.strict_json, b"[" * 13 + b"0" + b"]" * 13, code="json-depth")
        self.assertEqual(len(C.strict_json(dump({str(i): 0 for i in range(32)}))), 32)
        self.refuses(C.strict_json, dump({str(i): 0 for i in range(33)}), code="json-members")
        self.assertEqual(len(C.strict_json(dump([0] * 4096))), 4096)
        self.refuses(C.strict_json, dump([0] * 4097), code="json-items")
        self.assertEqual(len(C.strict_json(b'"' + b"\\u0061" * 4096 + b'"')), 4096)
        self.refuses(C.strict_json, b'"' + b"\\u0061" * 4097 + b'"', code="json-string")
        self.assertEqual(len(C.string("x" * 4096)), 4096)
        self.refuses(C.string, "x" * 4097, code="string")
        self.assertEqual(C.strict_json(b"999999999999"), 999999999999)
        self.refuses(C.strict_json, b"9999999999999", code="json-integer")

    def test_file_manifest_and_line_boundaries(self) -> None:
        source = self.fresh()
        source.write_bytes(b"x" * 524288)
        self.assertEqual(len(C.read_regular(str(source), 524288)), 524288)
        source.write_bytes(b"x" * 524289)
        self.refuses(C.read_regular, str(source), 524288, code="input-size")
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        payload = manifest.read_bytes()
        manifest.write_bytes(payload + b" " * (65536 - len(payload)))
        self.assertEqual(C.collect(str(manifest))["record_count"], 3)
        manifest.write_bytes(manifest.read_bytes() + b" ")
        self.refuses(C.collect, str(manifest), code="input-size")
        valid = b"x" * 8192 + b"\n" + self.fixture("nts.md")
        self.assertEqual(self.collect([("nts", C.NTS, valid)])[0], 0)
        self.assertEqual(self.collect([("nts", C.NTS, b"x" + valid)])[1]["sources"][0]["error"], "text-line")

    def test_source_address_and_reference_count_boundaries(self) -> None:
        entries = [("source-" + str(i), C.NTS, self.fixture("nts.md")) for i in range(8)]
        self.assertEqual(self.collect(entries)[1]["record_count"], 24)
        manifest = self.manifest(entries + [("source-nine", C.NTS, self.fixture("nts.md"))])
        self.refuses(C.collect, str(manifest), code="manifest")
        raw = rough_row()
        raw["addresses"] *= 8
        self.assertEqual(len(self.one_rough([raw])[1]["sources"][0]["records"][0]["meaning"]["endpoints"]), 8)
        raw["addresses"].append(raw["addresses"][0])
        self.assertEqual(self.one_rough([raw])[0], 3)
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        value = json.loads(manifest.read_bytes())
        related = value["sources"][0]["origin"]["related"]
        related.extend({"kind": "generator", "uri": "https://evidence.invalid/generator", "git_blob": None}
                       for _ in range(8))
        manifest.write_bytes(dump(value))
        self.assertEqual(C.collect(str(manifest))["record_count"], 3)
        related.append(related[0])
        manifest.write_bytes(dump(value))
        self.refuses(C.collect, str(manifest))

    def test_experimental_material_length_boundary(self) -> None:
        self.assertEqual(C.key_material("ml-dsa-44", "A" * 2731 + "=")["bytes"], 2048)
        self.refuses(C.key_material, "ml-dsa-44", "A" * 2732, code="key")
        self.refuses(C.key_material, "ed25519", "A" * 44, code="key")


class WorkLimitTests(Case):
    def test_record_and_aggregate_record_boundaries(self) -> None:
        rows = [rough_row() for _ in range(1024)]
        body = dump({"servers": rows})
        status, value, _ = self.collect([("rough-a", C.ROUGH, body), ("rough-b", C.ROUGH, body)])
        self.assertEqual((status, value["record_count"]), (0, 2048))
        manifest = self.manifest([("rough-a", C.ROUGH, body), ("rough-b", C.ROUGH, body),
                                  ("rough-c", C.ROUGH, dump({"servers": [rough_row()]}))])
        self.refuses(C.collect, str(manifest), code="aggregate-records")
        over = dump({"servers": rows + [rough_row()]})
        result = self.collect([("rough", C.ROUGH, over)])[1]
        self.assertEqual(result["sources"][0]["error"], "record-limit")
        self.assertEqual(result["record_count"], 0)

    def test_source_aggregate_byte_boundary(self) -> None:
        fixture = self.fixture("nts.md")
        padding = b"\n" * (524288 - len(fixture))
        body = fixture + padding
        entries = [("source-" + str(i), C.NTS, body) for i in range(4)]
        self.assertEqual(self.collect(entries)[1]["record_count"], 12)
        manifest = self.manifest(entries + [("over", C.NTS, b"x")])
        self.refuses(C.collect, str(manifest), code="aggregate-bytes")

    def test_json_work_token_boundary(self) -> None:
        # 1 root + 125 arrays + 499874 numbers = 500000 lexical tokens.
        value = [[0] * 4000 for _ in range(124)] + [[0] * 3874]
        self.assertEqual(len(C.strict_json(dump(value))), 125)
        value[-1].append(0)
        self.refuses(C.strict_json, dump(value), code="json-tokens")

    def test_encoded_output_byte_boundary(self) -> None:
        self.assertEqual(len(C.encoded("a" * 16777213)), 16777216)
        self.refuses(C.encoded, "a" * 16777214, code="output-size")


class PublicationTests(Case):
    def test_postpublication_stdout_failure_preserves_committed_status(self) -> None:
        class BrokenWriter(io.StringIO):
            attempts = 0

            def write(self, value):
                self.attempts += 1
                raise BrokenPipeError("controlled stdout failure")

        _, _, healthy = self.one_rough()
        previous = healthy.read_bytes()
        for expected, body in ((0, self.fixture("nts.md")), (3, b"incomplete source")):
            with self.subTest(expected=expected):
                manifest = self.manifest([("nts", C.NTS, body), ("ntp", C.NTP, self.fixture("ntp.md"))])
                output = self.fresh()
                broken, errors = BrokenWriter(), io.StringIO()
                with contextlib.redirect_stdout(broken), contextlib.redirect_stderr(errors):
                    actual = C.main(["collect", "--manifest", str(manifest), "--output", str(output)])
                self.assertGreater(broken.attempts, 0)
                self.assertEqual(json.loads(output.read_bytes())["status"],
                                 "complete-candidate" if expected == 0 else "incomplete-candidate")
                self.assertEqual(healthy.read_bytes(), previous)
                self.assertEqual(actual, expected, "diagnostic failure changed an already committed result")

    def test_postpublication_stderr_failure_preserves_committed_status(self) -> None:
        class BrokenWriter(io.StringIO):
            attempts = 0

            def write(self, value):
                self.attempts += 1
                raise OSError("controlled stderr failure")

        _, _, healthy = self.one_rough()
        previous = healthy.read_bytes()
        manifest = self.manifest([("nts", C.NTS, b"incomplete source"), ("ntp", C.NTP, self.fixture("ntp.md"))])
        output = self.fresh()
        broken, messages = BrokenWriter(), io.StringIO()
        with mock.patch.object(C.os, "unlink", side_effect=PermissionError("controlled cleanup failure")):
            with contextlib.redirect_stdout(messages), contextlib.redirect_stderr(broken):
                try:
                    actual = C.main(["collect", "--manifest", str(manifest), "--output", str(output)])
                except OSError:
                    actual = "raised-OSError"
        self.assertGreater(broken.attempts, 0)
        self.assertEqual(json.loads(output.read_bytes())["status"], "incomplete-candidate")
        self.assertEqual(healthy.read_bytes(), previous)
        self.assertEqual(actual, 3, "warning failure changed an already committed result")

    def test_existing_output_is_preserved(self) -> None:
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        output = self.fresh()
        output.write_bytes(b"healthy previous evidence\n")
        status, stdout, stderr = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertEqual((status, stdout, stderr), (2, "", "refused: output-exists\n"))
        self.assertEqual(output.read_bytes(), b"healthy previous evidence\n")
        self.assertEqual(list(self.directory.glob(".catalogue-*.partial")), [])

    def test_reached_link_failure_preserves_healthy_neighbor(self) -> None:
        _, _, healthy = self.one_rough()
        previous = healthy.read_bytes()
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        output = self.fresh()
        reached = []

        def failing_link(source, destination, **kwargs):
            staged = json.loads(Path(source).read_bytes())
            self.assertEqual(staged["status"], "complete-candidate")
            self.assertEqual(staged["record_count"], 3)
            reached.append(True)
            raise OSError("untrusted-not-printed")

        with mock.patch.object(C.os, "link", side_effect=failing_link):
            status, stdout, stderr = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertEqual((status, stdout, stderr), (2, "", "refused: output-publication\n"))
        self.assertEqual(reached, [True])
        self.assertFalse(output.exists())
        self.assertEqual(healthy.read_bytes(), previous)
        self.assertEqual(list(self.directory.glob(".catalogue-*.partial")), [])

    def test_postpublication_cleanup_failure_acknowledges_published_result(self) -> None:
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        output = self.fresh()
        with mock.patch.object(C.os, "unlink", side_effect=PermissionError("not-printed")):
            status, stdout, stderr = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertEqual((status, stdout, stderr),
                         (0, "published: complete-candidate\n", "warning: temporary-cleanup-after-publication\n"))
        self.assertEqual(json.loads(output.read_bytes())["record_count"], 3)
        partials = list(self.directory.glob(".catalogue-*.partial"))
        self.assertEqual(len(partials), 1)
        self.assertEqual(partials[0].read_bytes(), output.read_bytes())
        partials[0].unlink()

    def test_reached_file_sync_failure_leaves_no_final_output(self) -> None:
        manifest = self.manifest([("nts", C.NTS, self.fixture("nts.md"))])
        output = self.fresh()
        with mock.patch.object(C.os, "fsync", side_effect=OSError("not-printed")) as sync:
            status, _, error = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertEqual(sync.call_count, 1)
        self.assertEqual((status, error), (2, "refused: output-publication\n"))
        self.assertFalse(output.exists())
        self.assertEqual(list(self.directory.glob(".catalogue-*.partial")), [])


class OutputClosureTests(Case):
    def test_non_bmp_strings_remain_importable_at_decoded_limit(self) -> None:
        for count in (2049, 4096):
            with self.subTest(characters=count):
                replacement = ("\U0001f600" * count).encode("utf-8")
                body = self.fixture("nts.md").replace(b"Example Owner", replacement)
                status, value, output = self.collect([("nts", C.NTS, body)])
                self.assertEqual(status, 0)
                self.assertEqual(len(value["sources"][0]["records"][0]["raw"]["owner"]), count)
                self.assertEqual(self.difference(output, output)[0], 0)
        over = self.fixture("nts.md").replace(b"Example Owner", ("\U0001f600" * 4097).encode("utf-8"))
        status, value, _ = self.collect([("nts", C.NTS, over), ("ntp", C.NTP, self.fixture("ntp.md"))])
        self.assertEqual((status, value["record_count"]), (3, 3))

    def test_generated_token_overflow_refuses_before_publication(self) -> None:
        row = rough_row("a", "a.b:0")
        row["addresses"] *= 8
        _, _, healthy = self.one_rough([row])
        previous = healthy.read_bytes()
        body = dump({"servers": [row] * 1024})
        self.assertLessEqual(len(body), 524288)
        manifest = self.manifest([("rough-a", C.ROUGH, body), ("rough-b", C.ROUGH, body)])
        output = self.fresh()
        actual, _, _ = self.invoke(["collect", "--manifest", str(manifest), "--output", str(output)])
        self.assertEqual(healthy.read_bytes(), previous)
        C.load_snapshot(str(healthy), sha(previous))
        if output.exists():
            self.assertEqual(json.loads(output.read_bytes())["record_count"], 2048)
            self.refuses(C.load_snapshot, str(output), sha(output.read_bytes()), code="json-tokens")
        self.assertEqual(actual, 2, "an emitted candidate exceeded its own strict import boundary")
        self.assertFalse(output.exists())


if __name__ == "__main__":
    unittest.main(verbosity=2)

