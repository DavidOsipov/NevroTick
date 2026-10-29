#!/usr/bin/env python3
"""Bound a local MCP stdio process; namespace and payload admission belong to caller.

Only protocol frames reach stdout. Limits apply for the complete connection;
reconnect after exhausting a budget. Client deadlines alone do not stop native
work, so every transport failure and disconnect kills the child process group.
The caller must also provide a PID namespace with die-with-parent containment
for descendants that create their own sessions.
"""

from __future__ import annotations

import json
import math
import os
from pathlib import PurePosixPath
import selectors
import signal
import subprocess
import sys
import time


MAX_FRAME = 1024 * 1024
MAX_QUEUE = 4 * MAX_FRAME
MAX_OUTPUT = 64 * MAX_FRAME
MAX_STDERR = MAX_FRAME
MAX_PENDING = 32
MAX_CLIENT_DEPTH = 32
MAX_COLLECTION = 256
MAX_CLIENT_NODES = 4096
MAX_STRING_BYTES = 256 * 1024
REQUESTS = frozenset({"initialize", "ping", "tools/list", "tools/call",
                      "resources/list", "resources/read", "resources/templates/list",
                      "resources/subscribe", "resources/unsubscribe", "prompts/list",
                      "prompts/get", "logging/setLevel", "completion/complete"})
NOTIFICATIONS = frozenset({"notifications/initialized", "notifications/cancelled",
                          "notifications/progress"})
FORBIDDEN_KEYS = frozenset({"additionalargs", "command", "buildcommand",
                            "extractorenv", "evaluationfunction", "positional", "positionalargs"})
OUTPUT_KEYS = frozenset({"output", "interpretedoutput", "evaluationoutput", "logdir", "evaluatorlog"})
RESOURCE_LIMITS = {"threads": 2, "ram": 1024}
# The pinned upstream CLI adapter appends these arguments without `--` and
# unwraps JSON-array-looking `file` strings. Require canonical absolute inputs
# before either conversion can turn a supposed filename into another option.
CLI_POSITIONAL_PATHS = {
    "codeql_bqrs_decode": {"files": True},
    "codeql_bqrs_info": {"file": False},
    "codeql_bqrs_interpret": {"file": False},
    "codeql_query_compile": {"query": False},
    "codeql_query_run": {"query": False},
    "codeql_resolve_metadata": {"query": False},
    "codeql_resolve_queries": {"directory": False},
    "codeql_resolve_database": {"database": False},
    "codeql_resolve_tests": {"tests": True},
    "codeql_test_run": {"tests": True},
}
CLI_TOOLS = frozenset(CLI_POSITIONAL_PATHS) | {"codeql_resolve_languages", "codeql_resolve_packs"}


class ProtocolFailure(Exception):
    """A connection cannot continue inside its admitted envelope."""


def _object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ProtocolFailure("Duplicate JSON member.")
        result[key] = value
    return result


def _decode(frame):
    try:
        def nonfinite(_):
            raise ValueError("Non-finite JSON number.")
        value = json.loads(frame, object_pairs_hook=_object, parse_constant=nonfinite)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise ProtocolFailure("Malformed JSON protocol frame.") from error
    if not isinstance(value, dict) or value.get("jsonrpc") != "2.0":
        raise ProtocolFailure("Expected one JSON-RPC 2.0 object per line.")
    if "id" in value and type(value["id"]) not in (str, int):
        raise ProtocolFailure("Protocol identifiers must be strings or integers.")
    return value


def _argument_problem(value, depth=0):
    if depth > 64:
        return "Tool arguments exceed the nesting limit."
    if isinstance(value, dict):
        for key, nested in value.items():
            normalized = "".join(char for char in key.lower() if char.isalnum())
            if normalized in FORBIDDEN_KEYS:
                return "Tool execution override arguments are disabled."
            if normalized in {"learn", "overwrite"} and nested is not False:
                return "Learning and overwrite modes are disabled."
            if normalized in RESOURCE_LIMITS and (type(nested) is not int or not 1 <= nested <= RESOURCE_LIMITS[normalized]):
                return "Tool resources must be positive integers within threads=2 and ram=1024."
            if normalized in OUTPUT_KEYS:
                if not isinstance(nested, str) or "\0" in nested or "\\" in nested:
                    return "Output/log paths must be canonical absolute paths below /work/results."
                path = PurePosixPath(nested)
                if (not path.is_absolute() or path.as_posix() != nested or ".." in path.parts
                        or not path.is_relative_to("/work/results") or path == PurePosixPath("/work/results")):
                    return "Output/log paths must be canonical absolute paths below /work/results."
            problem = _argument_problem(nested, depth + 1)
            if problem:
                return problem
    elif isinstance(value, list):
        for nested in value:
            problem = _argument_problem(nested, depth + 1)
            if problem:
                return problem
    return None


def _client_problem(value):
    """Bound collections before the server's vulnerable schema validator sees them."""
    remaining = [(value, 0)]
    nodes = 0
    while remaining:
        item, depth = remaining.pop()
        nodes += 1
        if depth > MAX_CLIENT_DEPTH or nodes > MAX_CLIENT_NODES:
            return "Client request nesting/node budget exceeded."
        if isinstance(item, str) and len(item.encode("utf-8")) > MAX_STRING_BYTES:
            return "Client request string budget exceeded."
        if isinstance(item, dict):
            if len(item) > MAX_COLLECTION:
                return "Client request object budget exceeded."
            remaining.extend((key, depth + 1) for key in item)
            remaining.extend((nested, depth + 1) for nested in item.values())
        elif isinstance(item, list):
            if len(item) > MAX_COLLECTION:
                return "Client request array budget exceeded."
            remaining.extend((nested, depth + 1) for nested in item)
    return None


def _cli_problem(tool, arguments):
    if tool not in CLI_TOOLS:
        return None
    for key, array in CLI_POSITIONAL_PATHS.get(tool, {}).items():
        if key not in arguments:
            continue  # Missing required fields remain the server schema's error.
        values = arguments[key]
        if array:
            if not isinstance(values, list) or not values:
                return "CLI input paths must be a nonempty array."
        else:
            values = [values]
        for value in values:
            if not isinstance(value, str) or "\0" in value or "\\" in value:
                return "CLI positional inputs require canonical absolute namespace paths."
            path = PurePosixPath(value)
            if not path.is_absolute() or path.as_posix() != value or ".." in path.parts or value.startswith("//"):
                return "CLI positional inputs require canonical absolute namespace paths."
    remaining = [arguments]
    while remaining:
        value = remaining.pop()
        if isinstance(value, str) and (value.lstrip().startswith("-") or any(char in value for char in "\0\r\n")):
            return "CLI values cannot contain option-like or control-character arguments."
        if isinstance(value, dict):
            remaining.extend(value.values())
        elif isinstance(value, list):
            remaining.extend(value)
    return None


def _kill_group(process):
    # Kill even after the immediate child exits: its descendants may still live.
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    process.wait(timeout=5)


def _flush_client(fd, queued):
    """Deliver already validated frames after killing faulty native work."""
    deadline = time.monotonic() + 0.25
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(fd, selectors.EVENT_WRITE)
            while queued and time.monotonic() < deadline:
                if not selector.select(max(0, deadline - time.monotonic())):
                    break
                try:
                    written = os.write(fd, queued)
                except BlockingIOError:
                    continue
                del queued[:written]
    except OSError:
        pass  # A disconnected or blocked client cannot delay native cleanup.


def serve(argv, allowed_tools, environment, cwd, request_timeout=180, startup_timeout=240):
    """Proxy inherited client stdio; return native exit status or 1 on guard failure.

    `argv` must already implement the caller's hash admission and OS sandbox.
    This function deliberately accepts neither a shell command nor ambient env.
    """
    if (not isinstance(argv, (list, tuple)) or not argv
            or any(not isinstance(part, str) or not part or "\0" in part for part in argv)):
        raise ValueError("Expected a nonempty literal argv.")
    allowed = frozenset(allowed_tools)
    if not allowed or any(not isinstance(name, str) or not name for name in allowed):
        raise ValueError("Expected an explicit nonempty tool allowlist.")
    for limit in (request_timeout, startup_timeout):
        if isinstance(limit, bool) or not isinstance(limit, (int, float)) or not math.isfinite(limit) or limit <= 0:
            raise ValueError("Deadlines must be finite and positive.")
    if not isinstance(environment, dict):
        raise ValueError("An explicit environment is required.")

    process = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, cwd=cwd, env=environment,
                               start_new_session=True, bufsize=0)
    client_in, client_out = sys.stdin.fileno(), sys.stdout.fileno()
    server_in, server_out, server_err = process.stdin.fileno(), process.stdout.fileno(), process.stderr.fileno()
    buffers = {client_in: bytearray(), server_out: bytearray()}
    queues = {server_in: bytearray(), client_out: bytearray()}
    original_blocking = {fd: os.get_blocking(fd) for fd in (client_in, client_out)}
    pending = {}
    write_deadlines = {}
    output_bytes = 0
    stderr_bytes = 0
    initialized = False
    startup_deadline = time.monotonic() + startup_timeout
    client_closed = False
    server_closed = False
    native_status = None
    drain_deadline = None
    result = 1

    try:
        with selectors.DefaultSelector() as selector:
            for fd in (client_in, client_out, server_in, server_out, server_err):
                os.set_blocking(fd, False)
            for fd in (client_in, server_out, server_err):
                selector.register(fd, selectors.EVENT_READ)

            def enqueue(fd, value):
                frame = json.dumps(value, separators=(",", ":"), ensure_ascii=True,
                                   allow_nan=False).encode() + b"\n"
                if len(frame) > MAX_FRAME or len(queues[fd]) + len(frame) > MAX_QUEUE:
                    raise ProtocolFailure("Protocol output queue budget exceeded.")
                if not queues[fd]:
                    write_deadlines[fd] = time.monotonic() + request_timeout
                queues[fd].extend(frame)
                if fd not in selector.get_map():
                    selector.register(fd, selectors.EVENT_WRITE)

            def error_reply(message, code, text):
                if "id" in message:
                    enqueue(client_out, {"jsonrpc": "2.0", "id": message["id"],
                                         "error": {"code": code, "message": text}})

            def client_frame(message):
                problem = _client_problem(message)
                if problem:
                    error_reply(message, -32602, problem)
                    return
                method = message.get("method")
                if not isinstance(method, str):
                    raise ProtocolFailure("Unexpected client response or invalid method.")
                if "id" not in message:
                    if method in NOTIFICATIONS:
                        enqueue(server_in, message)
                    return
                identity = (type(message["id"]), message["id"])
                if identity in pending:
                    raise ProtocolFailure("Duplicate outstanding request identifier.")
                if method not in REQUESTS:
                    error_reply(message, -32601, "MCP method is disabled by local policy.")
                    return
                if method == "tools/call":
                    params = message.get("params", {})
                    if (not isinstance(params, dict) or not isinstance(params.get("name"), str)
                            or params["name"] not in allowed):
                        error_reply(message, -32601, "Tool is disabled by local policy.")
                        return
                    arguments = params.get("arguments", {})
                    problem = _argument_problem(arguments) if isinstance(arguments, dict) else "Tool arguments must be an object."
                    if not problem:
                        problem = _cli_problem(params["name"], arguments)
                    if problem:
                        error_reply(message, -32602, problem)
                        return
                    if params["name"] == "codeql_query_compile":
                        if (("warnings" in arguments and arguments["warnings"] != "error")
                                or ("dump-dil" in arguments and arguments["dump-dil"] is not False)):
                            error_reply(message, -32602, "Query compilation requires warnings=error and dump-dil=false.")
                            return
                        arguments["warnings"] = "error"
                        arguments["dump-dil"] = False
                        params["arguments"] = arguments
                if len(pending) >= MAX_PENDING:
                    error_reply(message, -32000, "Outstanding request limit reached.")
                    return
                timeout = startup_timeout if method == "initialize" else request_timeout
                pending[identity] = (method, time.monotonic() + timeout)
                enqueue(server_in, message)

            def server_frame(message):
                nonlocal initialized
                if "method" in message:
                    if not isinstance(message["method"], str):
                        raise ProtocolFailure("Invalid server method.")
                    if "id" in message:
                        # No server-originated host execution/sampling/elicitation.
                        enqueue(server_in, {"jsonrpc": "2.0", "id": message["id"],
                                            "error": {"code": -32601, "message": "Client requests are disabled by local policy."}})
                    else:
                        enqueue(client_out, message)
                    return
                if "id" not in message or ("result" in message) == ("error" in message):
                    raise ProtocolFailure("Malformed server response.")
                identity = (type(message["id"]), message["id"])
                if identity not in pending:
                    raise ProtocolFailure("Unsolicited server response.")
                method, _ = pending.pop(identity)
                if method == "tools/list" and "result" in message:
                    payload = message["result"]
                    if not isinstance(payload, dict) or not isinstance(payload.get("tools"), list):
                        raise ProtocolFailure("Malformed tool inventory.")
                    if any(not isinstance(tool, dict) or not isinstance(tool.get("name"), str) for tool in payload["tools"]):
                        raise ProtocolFailure("Malformed tool inventory entry.")
                    payload["tools"] = [tool for tool in payload["tools"] if tool["name"] in allowed]
                if method == "initialize" and "result" in message:
                    initialized = True
                enqueue(client_out, message)

            while True:
                now = time.monotonic()
                if any(now >= deadline for _, deadline in pending.values()) or (not initialized and now >= startup_deadline):
                    raise ProtocolFailure("MCP request/startup deadline exceeded; native work terminated.")
                if any(now >= deadline for deadline in write_deadlines.values()):
                    raise ProtocolFailure("MCP output delivery deadline exceeded; native work terminated.")
                observed_status = process.poll()
                if observed_status is not None and native_status is None:
                    native_status = observed_status
                    _kill_group(process)
                    drain_deadline = now + 2
                if native_status is not None and server_closed and not queues[client_out]:
                    result = native_status if native_status != 0 else (1 if pending else 0)
                    break
                if drain_deadline is not None and now >= drain_deadline:
                    raise ProtocolFailure("Child output did not drain after termination.")
                if client_closed and not pending and not queues[client_out] and not queues[server_in]:
                    result = native_status if native_status is not None else 0
                    break
                for key, event in selector.select(0.05):
                    fd = key.fd
                    if event & selectors.EVENT_WRITE:
                        try:
                            written = os.write(fd, queues[fd])
                        except BlockingIOError:
                            continue
                        del queues[fd][:written]
                        if not queues[fd]:
                            selector.unregister(fd)
                            write_deadlines.pop(fd, None)
                        continue
                    try:
                        chunk = os.read(fd, 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(fd)
                        if fd in buffers and buffers[fd]:
                            raise ProtocolFailure("Truncated JSON protocol frame.")
                        if fd == client_in:
                            client_closed = True
                        elif fd == server_out:
                            server_closed = True
                            if process.poll() is None:
                                drain_deadline = now + 2
                        continue
                    if fd == server_err:
                        stderr_bytes += len(chunk)
                        if stderr_bytes > MAX_STDERR:
                            raise ProtocolFailure("Server stderr output budget exceeded.")
                        # Do not copy arbitrary child logs or secrets to client output.
                        continue
                    if fd == server_out:
                        output_bytes += len(chunk)
                        if output_bytes > MAX_OUTPUT:
                            raise ProtocolFailure("Server protocol output budget exceeded.")
                    buffers[fd].extend(chunk)
                    while b"\n" in buffers[fd]:
                        frame, _, remainder = buffers[fd].partition(b"\n")
                        buffers[fd] = bytearray(remainder)
                        if len(frame) + 1 > MAX_FRAME:
                            raise ProtocolFailure("Protocol frame budget exceeded.")
                        message = _decode(frame)
                        (client_frame if fd == client_in else server_frame)(message)
                    if len(buffers[fd]) >= MAX_FRAME:
                        raise ProtocolFailure("Unterminated protocol frame exceeded its budget.")
    except (ProtocolFailure, OSError, ValueError, RecursionError) as error:
        print(f"CodeQL MCP supervisor: {error}", file=sys.stderr)
        result = native_status if native_status not in (None, 0) else 1
    finally:
        _kill_group(process)
        if queues[client_out]:
            _flush_client(client_out, queues[client_out])
        for stream in (process.stdin, process.stdout, process.stderr):
            stream.close()
        for fd, blocking in original_blocking.items():
            os.set_blocking(fd, blocking)
    return result
