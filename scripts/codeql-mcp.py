#!/usr/bin/env python3
"""Admit and serve the pinned local CodeQL MCP in an offline namespace.

--check verifies installation identity without connecting. --create-database
extracts an owned Rust source tree; it does not create an application workspace.
"""

from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys

ROOT = Path(__file__).resolve().parents[1]


def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


GUARD = module("codeql_analysis_guard", "check-analysis.py")
RUNTIME = GUARD.RUNTIME
SUPERVISOR = module("codeql_mcp_supervisor", "codeql-mcp-supervisor.py")
Incomplete = GUARD.Incomplete
PROFILE = {"request_timeout_seconds": 300, "startup_timeout_seconds": 600,
           "cpu_seconds": 600, "address_space_bytes": 8589934592,
           "ram_hint_mb": 1024, "threads": 2}
FORBIDDEN_TOOLS = {"audit_clear_repo", "annotation_delete", "codeql_database_create",
                   "codeql_pack_install", "codeql_test_accept", "codeql_test_extract",
                   "create_codeql_query", "codeql_query_format"}
ALLOWED_TOOLS = {"codeql_resolve_languages", "codeql_resolve_packs", "codeql_resolve_database",
                 "codeql_resolve_metadata", "codeql_resolve_queries", "codeql_resolve_tests",
                 "codeql_query_compile", "codeql_query_run", "codeql_bqrs_info", "codeql_bqrs_decode",
                 "codeql_bqrs_interpret", "read_database_source", "list_codeql_databases",
                 "list_query_run_results", "codeql_lsp_diagnostics", "codeql_lsp_definition",
                 "codeql_lsp_references", "codeql_lsp_document_symbols", "search_ql_code", "codeql_test_run"}


def inventory(base, item, *, copy=None):
    original = GUARD.internal(base, item["root"])
    folder = RUNTIME._directory(copy, "Runtime pack copy") if copy else original
    payload = GUARD.read_once(GUARD.internal(base, item["path"]))
    import hashlib
    if hashlib.sha256(payload).hexdigest() != item["sha256"]:
        raise Incomplete("CodeQL payload inventory digest changed.")
    manifest = GUARD.decode(payload)
    if manifest["schema_version"] != 1 or manifest["installation"] != str(original):
        raise Incomplete("CodeQL inventory root/schema changed.")
    files, links = manifest["files"], manifest.get("symlinks", {})
    if not files or not isinstance(files, dict) or not isinstance(links, dict):
        raise Incomplete("Missing complete CodeQL payload inventory.")
    for name in (*files, *links):
        GUARD.relative(name)
    seen_files, seen_links = set(), set()
    jobs = []

    def inspect(path):
        info = path.lstat()
        relative = path.relative_to(folder).as_posix()
        if stat.S_ISDIR(info.st_mode):
            with os.scandir(path) as children:
                for child in children:
                    inspect(Path(child.path))
        elif stat.S_ISREG(info.st_mode):
            if relative not in files:
                raise Incomplete("Unlisted installed CodeQL file: " + relative)
            seen_files.add(relative)
            jobs.append((path, files[relative]))
        elif stat.S_ISLNK(info.st_mode):
            expected = links.get(relative)
            # Publisher inventories retain the literal relative link text.
            if isinstance(expected, dict):
                expected = expected.get("target")
            target = path.resolve(strict=True)
            if (os.readlink(path) != expected or not target.is_relative_to(folder)
                    or target.relative_to(folder).as_posix() not in files):
                raise Incomplete("CodeQL payload link identity/containment changed.")
            seen_links.add(relative)
        else:
            raise Incomplete("Special file in CodeQL payload.")

    inspect(folder)
    if seen_files != set(files) or seen_links != set(links):
        raise Incomplete("Installed CodeQL payload is incomplete.")

    def check(job):
        path, expected = job
        if GUARD.digest(path) != expected:
            raise Incomplete("Installed CodeQL file changed: " + str(path.relative_to(folder)))

    with ThreadPoolExecutor(max_workers=4) as pool:
        list(pool.map(check, jobs))
    return {"root": item["root"], "files": len(files), "links": len(links)}


def admit():
    if os.geteuid() == 0 or not sys.platform.startswith("linux"):
        raise Incomplete("Local CodeQL MCP requires an ordinary Linux user.")
    policy = GUARD.decode(GUARD.read_once(ROOT / "policy/codeql-mcp.json"))
    if (policy.get("schema_version") != 1 or policy.get("server_version") != "2.27.3"
            or policy.get("cli_version") != "2.27.3" or policy.get("rust_library") != "0.2.24"
            or policy.get("transport") != "stdio" or policy.get("ci_codeql") != "deferred"
            or policy.get("profile") != PROFILE):
        raise Incomplete("Unreviewed CodeQL version or execution profile.")
    allowed = policy.get("allowed_tools")
    if (not isinstance(allowed, list) or not allowed or len(allowed) != len(set(allowed))
            or any(not isinstance(x, str) for x in allowed) or set(allowed) != ALLOWED_TOOLS):
        raise Incomplete("Invalid or prohibited MCP tool selection.")
    record = GUARD.decode(GUARD.read_once(Path.home() / ".local/share/nevrotick/codeql-installation.json",
                                        maximum=4096, private=True))
    if record.get("schema_version") != 1 or record.get("status") != "installed":
        raise Incomplete("CodeQL installation is incomplete.")
    base = RUNTIME._directory(Path(record["root"]), "CodeQL payload")
    work = RUNTIME._directory(Path(record["work"]), "CodeQL work")
    RUNTIME._narrow(base)
    RUNTIME._owned_work(work, ROOT)
    if RUNTIME._overlap(base, work) or RUNTIME._overlap(base, ROOT):
        raise Incomplete("CodeQL payload, checkout and work must be separate.")
    reports = [inventory(base, item, copy=work / "packs/rust-tools" if item["root"] == "rust-tools" else None)
               for item in policy["inventories"]]
    if {x["root"] for x in reports} != {"server", "rust-tools", "cli/codeql"} or len(reports) != 3:
        raise Incomplete("Incomplete CodeQL inventory set.")
    if GUARD.digest(GUARD.internal(base, "runtime/node")) != policy["node_sha256"]:
        raise Incomplete("Node executable identity changed.")
    analysis = GUARD.decode(GUARD.read_once(ROOT / "policy/analysis-tools.json"))
    analysis_base, stable = GUARD.load_installation(
        Path.home() / ".local/share/nevrotick/analysis-installation.json", ROOT)
    stable_spec = next(x for x in analysis["inventories"] if x["root"] == "stable")
    GUARD.check_inventory(analysis_base, stable_spec, stable)
    settings = work / "policy/codeql/config"
    if GUARD.digest(settings) != policy["config_sha256"]:
        raise Incomplete("CodeQL resource/pack configuration changed.")
    link = work / "home/.config/codeql/config"
    if (not link.is_symlink() or os.readlink(link) != "../../../policy/codeql/config"
            or link.resolve(strict=True) != settings):
        raise Incomplete("CodeQL home configuration redirects from its admitted file.")
    RUNTIME._helpers(ROOT)
    return {"base": base, "work": work, "stable": stable, "policy": policy, "inventories": reports}


def check_results(work):
    """Prevent an admitted output path from aliasing query/policy inputs.

    The generic work guard intentionally permits internal file links. Results
    have a narrower contract: owned regular files/directories, without symlinks
    or hardlinks, so writing an output cannot replace another work-tree inode.
    """
    results = work / "results"
    if not results.exists() and not results.is_symlink():
        return
    if results.resolve(strict=True) != results or not results.is_dir():
        raise Incomplete("CodeQL results must be a canonical directory without aliases.")
    pending = [results]
    while pending:
        path = pending.pop()
        info = path.lstat()
        if info.st_uid != os.geteuid():
            raise Incomplete("CodeQL results contain an unowned entry.")
        if stat.S_ISDIR(info.st_mode):
            with os.scandir(path) as children:
                pending.extend(Path(child.path) for child in children)
        elif not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
            raise Incomplete("CodeQL results must contain no symlinks, hardlinks or special files.")


def command(state, argv, *, cwd=None, sealed=()):
    base, work, stable = (state[x] for x in ("base", "work", "stable"))
    for relative in ("results", "monitoring", "tmp/codeql"):
        (work / relative).mkdir(parents=True, exist_ok=True, mode=0o700)
    check_results(work)
    env = {"TRANSPORT_MODE": "stdio", "CODEQL_PATH": str(base / "cli/codeql/codeql"),
           "CODEQL_MCP_TMP_DIR": "/work/tmp/codeql", "CODEQL_QUERY_LOG_DIR": "/work/results",
           "CODEQL_MCP_WORKSPACE": "/work/queries", "CODEQL_MCP_WORKSPACE_FOLDERS": "/work/queries",
           "CODEQL_DATABASES_BASE_DIRS": "/work", "CODEQL_QUERY_RUN_RESULTS_DIRS": "/work/results",
           "MONITORING_STORAGE_LOCATION": "/work/monitoring", "ENABLE_MONITORING_TOOLS": "false",
           "ENABLE_ANNOTATION_TOOLS": "false", "DEBUG": "false",
           "JAVA_TOOL_OPTIONS": "-Xmx1024m -XX:ActiveProcessorCount=2",
           "RUSTC": str(stable / "bin/rustc"), "RUSTDOC": str(stable / "bin/rustdoc")}
    return RUNTIME.sandbox_command(ROOT, work, argv, readonly=[base, ROOT, stable],
                                  path_dirs=[base / "runtime", base / "cli/codeql", stable / "bin"],
                                  environment=env, cwd=cwd or work / "queries",
                                  sealed=[work / "policy", work / "packs", *sealed], cpu_seconds=PROFILE["cpu_seconds"],
                                  address_space_bytes=PROFILE["address_space_bytes"])


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--create-database", metavar="SOURCE")
    parser.add_argument("--name", help="new database directory name, under private work/databases")
    args = parser.parse_args(argv)
    try:
        state = admit()
        base, work = state["base"], state["work"]
        if args.check:
            if args.create_database or args.name:
                raise Incomplete("Choose admission or extraction, not both.")
            print(json.dumps({"status": "admitted", "inventories": state["inventories"],
                              "workspace": str(work), "namespace_workspace": "/work",
                              "known_advisories": state["policy"]["known_advisories"],
                              "ci_codeql": "deferred"}, indent=2))
            return 0
        if args.create_database:
            source = RUNTIME._directory(Path(args.create_database), "Rust source")
            if not (source.is_relative_to(ROOT) or source.is_relative_to(work / "fixtures")):
                raise Incomplete("Extract only this checkout or an owned fixture source tree.")
            if not all((source / item).is_file() for item in ("Cargo.toml", "Cargo.lock")):
                raise Incomplete("Actual Cargo manifest/lockfile required; never scaffold for extraction.")
            if not args.name or not re.fullmatch(r"[a-z][a-z0-9-]{0,63}", args.name):
                raise Incomplete("A new bounded database name is required.")
            destination = work / "databases" / args.name
            if destination.exists() or destination.is_symlink():
                raise Incomplete("Database destination must be new.")
            destination.parent.mkdir(exist_ok=True, mode=0o700)
            source_arg = str(Path("/work") / source.relative_to(work)) if source.is_relative_to(work) else str(source)
            sealed = [source] if source.is_relative_to(work) else []
            before = GUARD.input_snapshot(source.parent, [source])
            native = [str(base / "cli/codeql/codeql"), "database", "create", "/work/databases/" + args.name,
                      "--language=rust", "--build-mode=none", "--source-root=" + source_arg]
            result = RUNTIME.ENV.run(command(state, native, sealed=sealed), cwd=ROOT, timeout=600,
                                     environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
                                     stdin=__import__("subprocess").DEVNULL)
            print(json.dumps(result, indent=2))
            if before != GUARD.input_snapshot(source.parent, [source]):
                raise Incomplete("Extraction source changed during the command.")
            return 0 if result["exit_code"] == 0 and result["failure"] is None else 1
        if args.name:
            raise Incomplete("--name applies only to extraction.")
        native = [str(base / "runtime/node"), str(base / "server/dist/codeql-development-mcp-server.js")]
        return SUPERVISOR.serve(command(state, native), state["policy"]["allowed_tools"],
                                environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"}, cwd=ROOT,
                                request_timeout=PROFILE["request_timeout_seconds"],
                                startup_timeout=PROFILE["startup_timeout_seconds"])
    except (Incomplete, OSError, ValueError, KeyError, TypeError, RuntimeError, StopIteration) as error:
        print("CodeQL MCP incomplete: " + str(error), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
