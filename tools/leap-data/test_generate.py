"""Independent LP-C01--03 fixtures for the offline leap-data boundary."""
from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("leap_data_generator", HERE / "generate.py")
assert SPEC is not None and SPEC.loader is not None
GEN = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = GEN
SPEC.loader.exec_module(GEN)
RAW = (HERE / "inputs/iana-2026f-leap-seconds.list").read_bytes()
BASELINE = b"2272060800 10\n"


def with_checksum(raw: bytes) -> bytes:
    """Independent checksum fixture using only documented physical-line rules."""
    digits = bytearray()
    for line in raw.splitlines():
        if line.startswith((b"#$", b"#@")):
            text = line[2:]
        elif line and line[0:1].isdigit():
            text = line
        else:
            continue
        digits.extend(c for c in text.split(b"#", 1)[0] if 48 <= c <= 57)
    digest = hashlib.sha1(digits, usedforsecurity=False).hexdigest()
    return raw + b"#h " + b" ".join(digest[i:i+8].encode() for i in range(0, 40, 8)) + b"\n"


class ParserTests(unittest.TestCase):
    def test_pinned_literal_history_and_optional_metadata(self):
        ledger = GEN.parse_leap_list(RAW)
        self.assertEqual(ledger.records[0], (2272060800, 10))
        self.assertEqual(ledger.records[-2:], ((3644697600, 36), (3692217600, 37)))
        self.assertEqual(len(ledger.records), 28)
        self.assertEqual(ledger.updated, 3992312697)
        self.assertEqual(ledger.expires, 4023129600)
        self.assertEqual(ledger.checksum, "a9bad14584c31c70758402aab37bfd545923836a")

    def test_empty_healthy_rejection(self):
        with self.assertRaisesRegex(GEN.GeneratorError, "no ledger records"):
            GEN.parse_leap_list(b"")

    def test_checksum_preserves_lexemes_and_ignores_signs_comments(self):
        raw = b"#$ 0000000000 # 99\n#@ 6311433600\n02272060800 +10 # 8\n2287785600 +11\n"
        ledger = GEN.parse_leap_list(with_checksum(raw))
        self.assertEqual(ledger.records, ((2272060800, 10), (2287785600, 11)))
        self.assertEqual(ledger.updated, 0)
        self.assertEqual(ledger.expires, 6311433600)
        self.assertIsNotNone(ledger.checksum)

    def test_absence_explicit_zero_and_final_line_crlf(self):
        absent = GEN.parse_leap_list(BASELINE.rstrip(b"\n"))
        zero = GEN.parse_leap_list(b"#@ 0\r\n" + BASELINE.replace(b"\n", b"\r\n"))
        self.assertIsNone(absent.updated)
        self.assertIsNone(absent.expires)
        self.assertIsNone(absent.checksum)
        self.assertEqual(zero.expires, 0)
        self.assertEqual(absent.records, zero.records)
        self.assertIsNone(GEN.parse_leap_list(b"# ordinary #@ 0\n"+BASELINE).expires)

    def test_positive_and_synthetic_negative_events(self):
        ledger = GEN.parse_leap_list(BASELINE+b"2287785600 11\n2303683200 10\n")
        self.assertEqual(ledger.records, ((2272060800, 10), (2287785600, 11), (2303683200, 10)))

    def test_complete_ascii_grammar_rejects_hostile_neighbors(self):
        bad = [
            b"\x00", b"\x7f", b"\x80", b"\x0b", b"\r",
            b" "+BASELINE, b"\t"+BASELINE, b"2272060800 10 junk\n",
            b"+2272060800 10\n", b"0x87654321 10\n", b"2272060800 infinity\n",
            b"2272060800 1_0\n", b"2272060800 10#comment\njunk\n",
            b"2272060800 10\n#@ infinity\n", b"#@ 0 1\n"+BASELINE,
            b"#@\n"+BASELINE, b"#$\n"+BASELINE, b"#h\n"+BASELINE,
            b"#@x 0\n"+BASELINE, b"#$x 0\n"+BASELINE,
            b"#@ 0\n#@ 1\n"+BASELINE, b"#$ 0\n#$ 1\n"+BASELINE,
            b"#h 1 2 3 4 5\n#h 1 2 3 4 5\n"+BASELINE,
            b"#h 1 2 3 4\n"+BASELINE, b"#h 1 2 3 4 5 6\n"+BASELINE,
            b"#h 0x1 2 3 4 5\n"+BASELINE, b"#h -1 2 3 4 5\n"+BASELINE,
            b"#h 000000000 2 3 4 5\n"+BASELINE,
            b"#h 1 2 3 4 5 trailing\n"+BASELINE,
        ]
        for raw in bad:
            with self.subTest(raw=raw[:48]), self.assertRaises(GEN.GeneratorError):
                GEN.parse_leap_list(raw)

    def test_ledger_semantics_are_checked_without_checksum_masking(self):
        bad = [
            b"2272060800 11\n",
            b"2272060801 10\n",
            BASELINE+b"2272060800 11\n",
            BASELINE+b"2303683200 11\n2287785600 12\n",
            BASELINE+b"2287785600 10\n",
            BASELINE+b"2287785600 12\n",
            BASELINE+b"2287785601 11\n",
            BASELINE+b"2287872000 11\n",
            BASELINE+b"9223372036854775808 11\n",
            BASELINE+b"00000000002272060800 11\n",
            BASELINE+b"2287785600 2147483648\n",
            BASELINE+b"2287785600 -2147483649\n",
            BASELINE+b"2287785600 +00000000010\n",
        ]
        for raw in bad:
            with self.subTest(raw=raw[-50:]), self.assertRaises(GEN.GeneratorError):
                GEN.parse_leap_list(raw)

    def test_checksum_mismatch_and_mutation_rejected_beside_valid(self):
        valid = with_checksum(BASELINE+b"2287785600 11\n")
        GEN.parse_leap_list(valid)
        for raw in (valid.replace(b"11\n", b"09\n"), valid.replace(b"#h ", b"#h 0", 1)):
            with self.subTest(raw=raw[-60:]), self.assertRaises(GEN.GeneratorError):
                GEN.parse_leap_list(raw)

    def test_exact_byte_line_and_physical_line_limits(self):
        # 128 lines of 512 printable bytes plus separators adjusted to exact cap.
        raw = BASELINE + b"#"+b"x"*511+b"\n"
        GEN.parse_leap_list(raw)
        with self.assertRaises(GEN.GeneratorError):
            GEN.parse_leap_list(BASELINE+b"#"+b"x"*512+b"\n")
        raw = BASELINE + b"\n"*1023
        self.assertEqual(len(raw.splitlines()),1024)
        GEN.parse_leap_list(raw)
        with self.assertRaises(GEN.GeneratorError):
            GEN.parse_leap_list(raw+b"\n")
        lines = [BASELINE.rstrip(b"\n")]
        remaining = 65536 - len(BASELINE)
        while remaining:
            width=min(513,remaining)
            lines.append(b"#"+b"x"*(width-2) if width>=2 else b"")
            remaining-=width
        raw=b"\n".join(lines)+b"\n"
        self.assertEqual(len(raw),65536)
        GEN.parse_leap_list(raw)
        with self.assertRaises(GEN.GeneratorError):
            GEN.parse_leap_list(raw+b"\n")

    def test_timestamp_lexical_max_and_wide_calendar_neighbors(self):
        for spelling in (b"2272060800 +10\n", b"02272060800 010\n",
                         b"0000000002272060800 +0000000010\n"):
            self.assertEqual(GEN.parse_leap_list(spelling).records, ((2272060800, 10),))
        for when in (6311433600, 255611289600):  # 2100-01-01 and 10000-01-01.
            self.assertEqual(GEN.parse_leap_list(BASELINE+f"{when} 11\n".encode()).records[-1],
                             (when,11))
            for invalid in (when+1, when+86400):
                with self.subTest(when=invalid),self.assertRaisesRegex(GEN.GeneratorError,"monthly"):
                    GEN.parse_leap_list(BASELINE+f"{invalid} 11\n".encode())
        with self.assertRaisesRegex(GEN.GeneratorError,"timestamp range"):
            GEN.parse_leap_list(BASELINE+b"#@ 9223372036854775808\n")
        with self.assertRaisesRegex(GEN.GeneratorError,"offset range"):
            GEN.parse_leap_list(BASELINE+b"2287785600 2147483648\n")

    def test_short_checksum_words_and_trailing_comment(self):
        # SHA1 fixture words are independent hex lexemes, not fixed width text.
        raw=with_checksum(BASELINE)
        line=raw.splitlines()[-1].split()
        words=[format(int(word,16),"x").encode() for word in line[1:]]
        candidate=BASELINE+b"#h "+b"\t".join(words)+b" # ignored 12345\n"
        self.assertEqual(GEN.parse_leap_list(candidate).records,((2272060800,10),))

    def test_128_records_neighbor_129_and_wide_months(self):
        # Independent year/month enumeration, first days starting January 1972.
        from datetime import datetime, timezone
        epoch=datetime(1900,1,1,tzinfo=timezone.utc)
        rows=[]
        for i in range(129):
            year=1972+i//12
            month=i%12+1
            seconds=int((datetime(year,month,1,tzinfo=timezone.utc)-epoch).total_seconds())
            rows.append(f"{seconds} {10+i}\n".encode())
        self.assertEqual(len(GEN.parse_leap_list(b"".join(rows[:128])).records),128)
        with self.assertRaises(GEN.GeneratorError):
            GEN.parse_leap_list(b"".join(rows))
        ledger=GEN.parse_leap_list(BASELINE+b"#@ 9223372036854775807\n")
        self.assertEqual(ledger.expires,9223372036854775807)


class ReviewedGenerationTests(unittest.TestCase):
    def fixture(self, path: Path, raw: bytes = RAW):
        manifest=json.loads((HERE/"provenance.json").read_text())
        (path/"inputs").mkdir()
        (path/"inputs/iana-2026f-leap-seconds.list").write_bytes(raw)
        (path/"LICENSE.iana").write_bytes((HERE/"LICENSE.iana").read_bytes())
        manifest["raw_input"]["sha256"]=hashlib.sha256(raw).hexdigest()
        manifest["raw_input"]["bytes"]=len(raw)
        lines=raw.splitlines()
        manifest["raw_input"]["physical_lines"]=len(lines)
        manifest["raw_input"]["maximum_line_bytes"]=max(map(len,lines),default=0)
        (path/"provenance.json").write_text(json.dumps(manifest))
        return path/"provenance.json",manifest

    def cli(self, *args):
        return subprocess.run([sys.executable,"-B",str(HERE/"generate.py"),*map(str,args)],
                              text=True,capture_output=True,check=False)

    def test_normalization_keeps_history_future_and_rule_separate(self):
        value=GEN.load_reviewed(HERE/"provenance.json")
        doc=json.loads(GEN.render_json(value))
        self.assertEqual(doc["baseline"],{"effective_ntp_seconds":2272060800,"tai_minus_utc_seconds":10})
        self.assertEqual(doc["transitions"][-1],{"effective_ntp_seconds":3692217600,
                         "tai_minus_utc_seconds":37,"delta_seconds":1})
        self.assertEqual(doc["history_complete_before_ntp_seconds"],3992284800)
        self.assertEqual(doc["finite_future_before_ntp_seconds"],4023129600)
        self.assertEqual(doc["continuous_from_ntp_seconds"],4019760000)
        self.assertFalse(any(row["effective_ntp_seconds"]==4019760000 for row in doc["transitions"]))
        self.assertEqual(GEN.render_json(value),GEN.render_json(GEN.load_reviewed(HERE/"provenance.json")))
        rust=GEN.render_rust(value)
        for token in (b"RECORDS: &[(u64, i32)]",b"HISTORY_COMPLETE_BEFORE: u64 = 3_992_284_800",
                      b"FINITE_FUTURE_BEFORE: Option<u64> = Some(4_023_129_600)",
                      b"CONTINUOUS_FROM: Option<u64> = Some(4_019_760_000)",b"RAW_SHA256: [u8; 32]",
                      b"PROVENANCE_SHA256: [u8; 32]",b"(2_272_060_800, 10)",b"(3_692_217_600, 37)"):
            self.assertIn(token,rust)
        self.assertTrue(rust.startswith(b"//! "))
        lines=rust.decode().splitlines()
        declarations=[i for i,line in enumerate(lines) if line.startswith("pub(super) const ")]
        self.assertEqual(len(declarations),7)
        self.assertTrue(all(lines[i-1].startswith("/// ") for i in declarations))
        self.assertLessEqual(len(rust),65536)

    def test_process_write_complete_check_and_atomic_failure(self):
        with tempfile.TemporaryDirectory() as td:
            root=Path(td);manifest,_=self.fixture(root)
            target=root/"generated.rs"
            args=["--provenance",manifest,"--format","rust","--output",target]
            wrote=self.cli(*args,"--write")
            self.assertEqual(wrote.returncode,0,wrote.stderr)
            before=target.read_bytes()
            self.assertEqual(self.cli(*args,"--check").returncode,0)
            target.write_bytes(before+b"// unexpected suffix\n")
            failed=self.cli(*args,"--check")
            self.assertNotEqual(failed.returncode,0)
            self.assertEqual(target.read_bytes(),before+b"// unexpected suffix\n")
            bad=(root/"inputs/iana-2026f-leap-seconds.list")
            bad.write_bytes(bad.read_bytes()+b"\x00")
            failed=self.cli(*args,"--write")
            self.assertNotEqual(failed.returncode,0)
            self.assertEqual(target.read_bytes(),before+b"// unexpected suffix\n")
            self.assertFalse(list(root.glob(".generated.rs.*")))
            self.assertLessEqual(len(failed.stderr.encode()),512)

    def test_process_semantic_failure_reaches_rule_and_preserves_previous(self):
        with tempfile.TemporaryDirectory() as td:
            root=Path(td)
            raw=b"\n".join(line for line in RAW.splitlines() if not line.startswith(b"#h"))+b"\n"
            raw=raw.replace(b"3692217600      37",b"3692217600      38")
            manifest,doc=self.fixture(root,raw)
            doc["raw_input"]["compatibility_checksum_sha1"]=None
            manifest.write_text(json.dumps(doc))
            target=root/"result.json";target.write_bytes(b"previous")
            result=self.cli("--provenance",manifest,"--format","json","--output",target,"--write")
            self.assertNotEqual(result.returncode,0)
            self.assertIn("offset step",result.stderr)
            self.assertEqual(target.read_bytes(),b"previous")

    def test_atomic_replace_failure_and_protected_input_output(self):
        from unittest import mock
        with tempfile.TemporaryDirectory() as td:
            root=Path(td);path,_=self.fixture(root)
            target=root/"generated.rs";target.write_bytes(b"previous")
            with mock.patch.object(GEN.os,"replace",side_effect=OSError("injected replace failure")):
                with self.assertRaises(OSError):
                    GEN.atomic_write(target,b"replacement")
            self.assertEqual(target.read_bytes(),b"previous")
            self.assertFalse(list(root.glob(".generated.rs.*")))
            before=path.read_bytes()
            result=self.cli("--provenance",path,"--format","json","--output",path,"--write")
            self.assertNotEqual(result.returncode,0)
            self.assertIn("pinned input",result.stderr)
            self.assertEqual(path.read_bytes(),before)
            target.unlink();target.symlink_to(path)
            result=self.cli("--provenance",path,"--output",target,"--write")
            self.assertNotEqual(result.returncode,0)
            self.assertEqual(path.read_bytes(),before)

    def test_process_json_output_literal_data_and_pin_identities(self):
        with tempfile.TemporaryDirectory() as td:
            target=Path(td)/"normalized.json"
            result=self.cli("--format","json","--write","--output",target)
            self.assertEqual(result.returncode,0,result.stderr)
            data=json.loads(target.read_bytes())
            self.assertEqual(data["identity"]["raw_sha256"],
                             "db5a895f16853b03bfc865e8d68f9fc8710ef1740e3400c701cd46a5bbbc3433")
            self.assertEqual(data["identity"]["provenance_sha256"],
                             "76043c66a9f737bc27492125cd9d6ed118137d13387803e77f38a4e93aa062cd")
            self.assertEqual(len(data["transitions"]),27)
            self.assertEqual(data["transitions"][0],{"delta_seconds":1,
                             "effective_ntp_seconds":2287785600,"tai_minus_utc_seconds":11})
            self.assertEqual(self.cli("--format","json","--check","--output",target).returncode,0)

    def test_reviewed_metadata_never_silently_falls_back(self):
        for kind in ("missing_metadata_claim","unreviewed_history","conflicting_rule","wrong_utc",
                     "traversal","missing_licence","oversized_manifest"):
            with self.subTest(kind=kind),tempfile.TemporaryDirectory() as td:
                root=Path(td);path,doc=self.fixture(root)
                if kind=="missing_metadata_claim":
                    doc["raw_input"]["expires_before_ntp_seconds"]=None
                elif kind=="unreviewed_history":
                    doc["history_review"]["complete_before_ntp_seconds"]=0
                    doc["history_review"]["complete_before_utc"]="1900-01-01T00:00:00Z"
                elif kind=="conflicting_rule":
                    doc["selected_continuous_rule"]["effective_ntp_seconds"]=3692217600
                    doc["selected_continuous_rule"]["effective_utc"]="2017-01-01T00:00:00Z"
                elif kind=="wrong_utc":
                    doc["history_review"]["complete_before_utc"]="2026-07-07T00:00:00Z"
                elif kind=="traversal":
                    doc["raw_input"]["path"]="../iana-2026f-leap-seconds.list"
                elif kind=="missing_licence":
                    (root/"LICENSE.iana").unlink()
                path.write_text(json.dumps(doc) if kind!="oversized_manifest" else " "*16385)
                with self.assertRaises((GEN.GeneratorError,OSError)):
                    GEN.load_reviewed(path)

    def test_strict_manifest_unknown_duplicate_types_and_pins(self):
        with tempfile.TemporaryDirectory() as td:
            root=Path(td);manifest,doc=self.fixture(root)
            doc["status"]="pinned-source-inputs-only; no product or release qualification"
            manifest.write_text(json.dumps(doc))
            GEN.load_reviewed(manifest)  # Healthy static-role neighbor.
            variants=[]
            v=json.loads(json.dumps(doc));v["status"]="pinned-source-inputs-only; generator and Rust knowledge implementation pending";variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["status"]="product-qualified";variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["surprise"]=1;variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["raw_input"]["sha256"]="0"*64;variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["licence"]["sha256"]="0"*64;variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["limits"]["records_including_baseline"]=True;variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));del v["history_review"]["complete_before_ntp_seconds"];variants.append(json.dumps(v))
            v=json.loads(json.dumps(doc));v["selected_continuous_rule"]["effective_ntp_seconds"]=True;variants.append(json.dumps(v))
            variants.extend([manifest.read_text().replace('"schema_version": 1','"schema_version": 1, "schema_version": 1'),
                             "["*2000+"0"+"]"*2000])
            for text in variants:
                manifest.write_text(text)
                with self.subTest(text=text[:40]), self.assertRaises(GEN.GeneratorError):
                    GEN.load_reviewed(manifest)

    def test_missing_expiry_and_zero_are_not_infinite_or_fabricated(self):
        for marker,expected in ((None,None),(b"#@ 0",0)):
            with self.subTest(marker=marker),tempfile.TemporaryDirectory() as td:
                root=Path(td)
                raw=b"\n".join(line for line in RAW.splitlines() if not line.startswith((b"#@",b"#h")))+b"\n"
                if marker is not None:raw+=marker+b"\n"
                path,doc=self.fixture(root,raw)
                doc["raw_input"]["expires_before_ntp_seconds"]=expected
                doc["raw_input"]["compatibility_checksum_sha1"]=None
                path.write_text(json.dumps(doc))
                normalized=json.loads(GEN.render_json(GEN.load_reviewed(path)))
                self.assertEqual(normalized["finite_future_before_ntp_seconds"],expected)
                self.assertEqual(normalized["history_complete_before_ntp_seconds"],3992284800)

    def test_process_fifo_inputs_and_check_output_reject_without_waiting(self):
        import os
        for role in ("provenance","raw","licence","check_output"):
            with self.subTest(role=role),tempfile.TemporaryDirectory() as td:
                root=Path(td);manifest,_=self.fixture(root)
                target=root/"generated.rs"
                selected={"provenance":manifest,"raw":root/"inputs/iana-2026f-leap-seconds.list",
                          "licence":root/"LICENSE.iana","check_output":target}[role]
                selected.unlink(missing_ok=True);os.mkfifo(selected)
                result=subprocess.run([sys.executable,"-B",str(HERE/"generate.py"),
                    "--provenance",str(manifest),"--output",str(target),"--check"],
                    capture_output=True,text=True,timeout=1,check=False)
                self.assertNotEqual(result.returncode,0)
                self.assertIn("regular local input",result.stderr)

    def test_bounded_read_asks_for_only_limit_plus_one(self):
        import io
        class Tracking(io.BytesIO):
            requested=[]
            def read(self,size=-1):
                self.requested.append(size)
                return super().read(size)
        stream=Tracking(b"x"*65537)
        with self.assertRaises(GEN.GeneratorError):
            GEN.read_bounded(stream,65536)
        self.assertEqual(stream.requested,[65537])


if __name__=="__main__":
    unittest.main()
