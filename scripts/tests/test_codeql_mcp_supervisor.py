"""Behavioral stdio and process-custody controls; no real analyzer is executed."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest


ROOT = Path(__file__).resolve().parents[2]
SUPERVISOR = ROOT / "scripts/codeql-mcp-supervisor.py"
SPEC = importlib.util.spec_from_file_location("codeql_mcp_supervisor", SUPERVISOR)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

FAKE = '''import json,os,pathlib,subprocess,sys,time
mode=sys.argv[1]
record=pathlib.Path(sys.argv[2])
if mode == "native": sys.exit(17)
if mode == "startup_timeout": time.sleep(60)
for line in sys.stdin:
 message=json.loads(line)
 with record.open("a") as output: output.write(json.dumps(message)+"\\n")
 if "id" not in message: continue
 method=message["method"]
 if method == "initialize":
  result={"protocolVersion":"2024-11-05","capabilities":{},"serverInfo":{"name":"fixture","version":"1"}}
 elif method == "tools/list":
  result={"tools":[{"name":"inspect","inputSchema":{"type":"object"}},{"name":"danger","inputSchema":{"type":"object"}}]}
 elif method == "tools/call":
  if mode in {"timeout","native_child"}:
   child=subprocess.Popen([sys.executable,"-c","import time; time.sleep(60)"])
   record.with_suffix(".pid").write_text(str(child.pid))
   if mode == "native_child": sys.exit(23)
   time.sleep(60)
  if mode == "bad": print("not protocol",flush=True); time.sleep(60)
  if mode == "large": print("x"*(1024*1024+1),flush=True); time.sleep(60)
  if mode == "stderr": os.write(2,b"x"*(1024*1024+1)); time.sleep(60)
  if mode == "error":
   print(json.dumps({"jsonrpc":"2.0","id":message["id"],"error":{"code":-32001,"message":"native tool error"}}),flush=True)
   continue
  result={"content":[{"type":"text","text":"healthy"}],"isError":False}
  if mode == "blocked_client":
   child=subprocess.Popen([sys.executable,"-c","import time; time.sleep(60)"])
   record.with_suffix(".pid").write_text(str(child.pid))
   result["content"][0]["text"]="x"*(256*1024)
 else: result={}
 print(json.dumps({"jsonrpc":"2.0","id":message["id"],"result":result}),flush=True)
'''


def request(identity, method, params=None):
    result = {"jsonrpc": "2.0", "id": identity, "method": method}
    if params is not None:
        result["params"] = params
    return result


class SupervisorControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-mcp-protocol-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.fake = self.work / "fake.py"
        self.fake.write_text(FAKE)
        self.record = self.work / "calls.jsonl"

    def launch(self, messages, mode="healthy", timeout=0.7, raw=None, limits=None, startup_timeout=2,
               allowed_tools=None):
        source = (
            "import importlib.util,sys; "
            f"s=importlib.util.spec_from_file_location('supervisor',{str(SUPERVISOR)!r}); "
            "m=importlib.util.module_from_spec(s); s.loader.exec_module(m); "
        )
        for name, value in (limits or {}).items():
            source += f"m.{name}={value!r}; "
        source += (
            f"sys.exit(m.serve({[sys.executable, '-B', str(self.fake), mode, str(self.record)]!r},"
            f"{allowed_tools or ['inspect']!r},{{'PATH':'/usr/bin:/bin','LANG':'C.UTF-8'}},{str(self.work)!r},"
            f"request_timeout={timeout!r},startup_timeout={startup_timeout!r}))"
        )
        data = raw if raw is not None else b"".join(json.dumps(message).encode() + b"\n" for message in messages)
        completed = subprocess.run([sys.executable, "-B", "-c", source], input=data,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=8)
        self.assertNotIn(b"Traceback", completed.stderr)
        frames = [json.loads(line) for line in completed.stdout.splitlines()]
        calls = [json.loads(line) for line in self.record.read_text().splitlines()] if self.record.exists() else []
        return completed, frames, calls

    def standard(self, arguments=None):
        return [request(1, "initialize"),
                {"jsonrpc": "2.0", "method": "notifications/initialized"},
                request(2, "tools/list"),
                request(3, "tools/call", {"name": "inspect", "arguments": arguments or {}})]

    def assert_child_stopped(self):
        pid = int(self.record.with_suffix(".pid").read_text())
        deadline = time.monotonic() + 2
        while time.monotonic() < deadline:
            try:
                state = Path(f"/proc/{pid}/stat").read_text().split(")", 1)[1].split()[0]
            except FileNotFoundError:
                return
            if state == "Z":  # Killed; this test cannot reap another parent's child.
                return
            time.sleep(0.02)
        self.fail(f"Descendant {pid} still executes after supervisor shutdown")

    def test_healthy_protocol_filters_inventory_preserves_results_and_notifications(self):
        completed, frames, calls = self.launch(self.standard())
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len(frames), 3)
        by_id = {message["id"]: message for message in frames}
        self.assertEqual([tool["name"] for tool in by_id[2]["result"]["tools"]], ["inspect"])
        self.assertEqual(by_id[3]["result"]["content"][0]["text"], "healthy")
        self.assertIn("notifications/initialized", [message["method"] for message in calls])

    def test_disabled_tool_never_reaches_server_and_healthy_tool_still_runs(self):
        messages = self.standard()
        messages.append(request(4, "tools/call", {"name": "danger", "arguments": {}}))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(next(frame for frame in frames if frame["id"] == 4)["error"]["code"], -32601)
        self.assertEqual([call["params"]["name"] for call in calls if call["method"] == "tools/call"], ["inspect"])

    def test_execution_overrides_rejected_recursively_before_server(self):
        values = [{"additionalArgs": []}, {"additional_args": ""}, {"learn": True},
                  {"overwrite": True}, {"command": "id"}, {"buildCommand": "id"},
                  {"extractorEnv": {}}, {"evaluationFunction": "eval"},
                  {"nested": [{"additional_args": ["--escape"]}]}]
        messages = self.standard({"learn": False, "overwrite": False})
        messages.extend(request(index + 10, "tools/call", {"name": "inspect", "arguments": value})
                        for index, value in enumerate(values))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if "error" in frame]), len(values))
        self.assertEqual(len([call for call in calls if call["method"] == "tools/call"]), 1)

    def test_pre_schema_collection_depth_node_and_string_bounds(self):
        nested = "leaf"
        for _ in range(34):
            nested = [nested]
        values = [{"items": list(range(257))}, {str(index): 0 for index in range(257)},
                  {"nested": nested}, {"nodes": [[0] * 200 for _ in range(22)]},
                  {"text": "x" * (256 * 1024 + 1)}]
        messages = self.standard({"items": [1, 2], "nested": [["healthy"]]})
        messages.extend(request(index + 10, "tools/call", {"name": "inspect", "arguments": value})
                        for index, value in enumerate(values))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]), len(values))
        self.assertEqual(len([call for call in calls if call["method"] == "tools/call"]), 1)

    def test_outputs_logs_cannot_replace_inputs_or_escape_results(self):
        keys = ["output", "interpretedOutput", "evaluationOutput", "logDir", "evaluator-log"]
        invalid = ["/work/policy.json", "/work/results/../policy.json", "results/out.bqrs",
                   "/work/results//out.bqrs", "/work/results/./out.bqrs", "/work/results",
                   "//work/results/out.bqrs", "/work/results/logs/", None]
        messages = self.standard()
        for key in keys:
            messages.append(request(len(messages) + 10, "tools/call", {
                "name": "inspect", "arguments": {key: "/work/results/run-1/" + key}}))
        for key in keys:
            for value in invalid:
                messages.append(request(len(messages) + 10, "tools/call", {
                    "name": "inspect", "arguments": {"nested": {key: value}}}))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]),
                         len(keys) * len(invalid))
        self.assertEqual(len([call for call in calls if call["method"] == "tools/call"]), 1 + len(keys))

    def test_compile_enforces_strict_defaults_and_rejects_weaker_overrides(self):
        inputs = [{}, {"warnings": "error", "dump-dil": False}, {"warnings": "show"},
                  {"warnings": "hide"}, {"dump-dil": True}, {"dump-dil": "false"}]
        messages = [request(1, "initialize")]
        messages.extend(request(index + 10, "tools/call", {"name": "codeql_query_compile", "arguments": value})
                        for index, value in enumerate(inputs))
        completed, frames, calls = self.launch(messages, allowed_tools=["codeql_query_compile"])
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]), 4)
        forwarded = [call["params"]["arguments"] for call in calls if call["method"] == "tools/call"]
        self.assertEqual(forwarded, [{"warnings": "error", "dump-dil": False}] * 2)

    def test_call_resources_cannot_override_fixed_upper_bounds(self):
        healthy = [{"threads": 1, "ram": 1}, {"threads": 2, "ram": 1024}]
        rejected = [{key: value} for key, maximum in (("threads", 2), ("ram", 1024))
                    for value in (0, -1, maximum + 1, True, False, 1.0, "1", None)]
        messages = [request(1, "initialize")]
        messages.extend(request(index + 10, "tools/call", {"name": "inspect", "arguments": value})
                        for index, value in enumerate(healthy + rejected))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]), len(rejected))
        self.assertEqual([call["params"]["arguments"] for call in calls if call["method"] == "tools/call"], healthy)

    def test_cli_positional_paths_cannot_smuggle_flags_or_encoded_arrays(self):
        cases = [("codeql_bqrs_decode", "files", True), ("codeql_bqrs_info", "file", False),
                 ("codeql_bqrs_interpret", "file", False), ("codeql_query_compile", "query", False),
                 ("codeql_query_run", "query", False), ("codeql_resolve_metadata", "query", False),
                 ("codeql_resolve_queries", "directory", False), ("codeql_resolve_database", "database", False),
                 ("codeql_resolve_tests", "tests", True), ("codeql_test_run", "tests", True)]
        messages = [request(1, "initialize")]
        invalid = ["--output=/work/queries/test.expected", "-o/work/queries/test.expected",
                   '["--output=/work/queries/test.expected"]', "relative/query.ql", "/work/queries/../policy/config"]
        for tool, key, array in cases:
            value = ["/work/results/input.bqrs"] if array else "/work/queries/input.ql"
            messages.append(request(len(messages) + 10, "tools/call", {"name": tool, "arguments": {key: value}}))
            for bad in invalid:
                value = ["/work/results/input.bqrs", bad] if array else bad
                messages.append(request(len(messages) + 10, "tools/call", {"name": tool, "arguments": {key: value}}))
        completed, frames, calls = self.launch(messages, allowed_tools=[tool for tool, _, _ in cases])
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]), len(cases) * len(invalid))
        self.assertEqual(len([call for call in calls if call["method"] == "tools/call"]), len(cases))

    def test_cli_named_values_preserve_metadata_and_builtins_without_arg_escape(self):
        healthy = [("codeql_query_run", {"queryName": "PrintAST", "queryLanguage": "rust",
                                        "database": "/work/databases/fixture", "external": ["selected=/work/results/input.csv"]}),
                   ("codeql_bqrs_interpret", {"file": "/work/results/input.bqrs", "t": ["kind=problem", "id=fixture/test"]})]
        rejected = [("codeql_query_run", {"external": ["--output=/work/queries/test.expected"]}),
                    ("codeql_bqrs_interpret", {"file": "/work/results/input.bqrs", "t": ["--output=/work/queries/test.expected"]}),
                    ("codeql_query_run", {"_positional": ["--output=/work/queries/test.expected"]}),
                    ("codeql_query_run", {"positionalArgs": ["/work/queries/input.ql"]})]
        messages = [request(1, "initialize")]
        messages.extend(request(index + 10, "tools/call", {"name": tool, "arguments": arguments})
                        for index, (tool, arguments) in enumerate(healthy + rejected))
        completed, frames, calls = self.launch(messages, allowed_tools=["codeql_query_run", "codeql_bqrs_interpret"])
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len([frame for frame in frames if frame.get("error", {}).get("code") == -32602]), len(rejected))
        self.assertEqual([call["params"]["arguments"] for call in calls if call["method"] == "tools/call"],
                         [arguments for _, arguments in healthy])

    def test_native_protocol_error_is_preserved(self):
        completed, frames, _ = self.launch(self.standard(), mode="error")
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(frames[-1]["error"], {"code": -32001, "message": "native tool error"})

    def test_native_exit_status_is_preserved(self):
        completed, _, _ = self.launch([request(1, "initialize")], mode="native")
        self.assertEqual(completed.returncode, 17, completed.stderr)

    def test_request_deadline_stops_descendant(self):
        completed, frames, _ = self.launch(self.standard(), mode="timeout")
        self.assertEqual(completed.returncode, 1)
        self.assertIn(b"deadline", completed.stderr)
        self.assertEqual({frame["id"] for frame in frames}, {1, 2})
        self.assert_child_stopped()

    def test_startup_deadline_stops_unresponsive_server(self):
        completed, frames, _ = self.launch([request(1, "initialize")], mode="startup_timeout", startup_timeout=0.2)
        self.assertEqual(completed.returncode, 1)
        self.assertIn(b"deadline", completed.stderr)
        self.assertEqual(frames, [])

    def test_native_exit_also_stops_retained_descendant(self):
        completed, _, _ = self.launch(self.standard(), mode="native_child")
        self.assertEqual(completed.returncode, 23, completed.stderr)
        self.assert_child_stopped()

    def test_client_that_stops_reading_cannot_retain_native_work(self):
        source = (
            "import importlib.util,sys; "
            f"s=importlib.util.spec_from_file_location('supervisor',{str(SUPERVISOR)!r}); "
            "m=importlib.util.module_from_spec(s); s.loader.exec_module(m); "
            f"sys.exit(m.serve({[sys.executable, '-B', str(self.fake), 'blocked_client', str(self.record)]!r},"
            f"['inspect'],{{'PATH':'/usr/bin:/bin'}},{str(self.work)!r},request_timeout=0.2,startup_timeout=2))"
        )
        process = subprocess.Popen([sys.executable, "-B", "-c", source], stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            process.stdin.write(b"".join(json.dumps(message).encode() + b"\n" for message in self.standard()))
            process.stdin.close()
            self.assertEqual(process.wait(timeout=4), 1)
            self.assertIn(b"output delivery deadline", process.stderr.read())
            self.assert_child_stopped()
        finally:
            if process.poll() is None:
                process.kill()
                process.wait(timeout=2)
            process.stdin.close()
            process.stdout.close()
            process.stderr.close()

    def test_non_protocol_stdout_fails_without_forwarding_it(self):
        completed, frames, _ = self.launch(self.standard(), mode="bad")
        self.assertEqual(completed.returncode, 1)
        self.assertIn(b"Malformed JSON", completed.stderr)
        self.assertEqual({frame["id"] for frame in frames}, {1, 2})

    def test_unterminated_frame_and_stderr_are_bounded(self):
        for mode in ("large", "stderr"):
            with self.subTest(mode=mode):
                completed, _, _ = self.launch(self.standard(), mode=mode)
                self.assertEqual(completed.returncode, 1)
                self.assertIn(b"budget", completed.stderr)

    def test_duplicate_keys_and_nonfinite_input_are_rejected(self):
        for raw in (b'{"jsonrpc":"2.0","id":1,"method":"ping","method":"tools/call"}\n',
                    b'{"jsonrpc":"2.0","id":1,"method":"ping","params":{"n":NaN}}\n'):
            with self.subTest(raw=raw):
                completed, _, calls = self.launch([], raw=raw)
                self.assertEqual(completed.returncode, 1)
                self.assertEqual(calls, [])

    def test_protocol_total_output_budget_is_enforced(self):
        completed, _, _ = self.launch(self.standard(), limits={"MAX_OUTPUT": 64})
        self.assertEqual(completed.returncode, 1)
        self.assertIn(b"output budget", completed.stderr)

    def test_protocol_queued_output_budget_is_enforced(self):
        completed, _, _ = self.launch(self.standard(), limits={"MAX_QUEUE": 64})
        self.assertEqual(completed.returncode, 1)
        self.assertIn(b"queue budget", completed.stderr)

    def test_normal_protocol_methods_forward_without_tool_alias(self):
        methods = ["ping", "resources/list", "resources/read", "resources/templates/list",
                   "prompts/list", "prompts/get"]
        messages = [request(1, "initialize")]
        messages.extend(request(index + 10, method, {}) for index, method in enumerate(methods))
        completed, frames, calls = self.launch(messages)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(len(frames), len(messages))
        self.assertEqual([call["method"] for call in calls], ["initialize", *methods])


if __name__ == "__main__":
    unittest.main()
