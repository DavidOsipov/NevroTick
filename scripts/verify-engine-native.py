#!/usr/bin/env python3
"""Build and separately supervise bounded Engine Windows tests (GPL-2.0-only).

The build lane executes only admitted offline WSL tools. It grants no native
execution, installation, elevation, clock, service or security-setting effects.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SELF = "scripts/verify-engine-native.py"
LAUNCHER = "tools/windows-sandbox/gated-console-launcher.rs"
CPU = "tools/windows-sandbox/cpu-os-v3-admission.rs"
TARGET = "x86_64-pc-windows-msvc"


def load(name, relative):
    """Import one fixed repository helper without invoking its CLI."""
    specification = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


ALLOCATION = load("engine_native_allocation", "scripts/verify-time-domain-windows-allocation.py")
PRODUCT, ANALYSIS, ENV = ALLOCATION.PRODUCT, ALLOCATION.ANALYSIS, ALLOCATION.ENV
PE = load("engine_native_pe", "scripts/cross-link-service-fixture.py")
Incomplete = ENV.Incomplete
EXTRAS = (SELF, LAUNCHER, CPU, "scripts/verify-time-domain-windows-allocation.py",
          "scripts/cross-link-service-fixture.py", "tools/windows-service-fixture/qualification.py",
          "scripts/portable-analysis.py", "scripts/prepare-product.py", "scripts/check-analysis.py", "scripts/analysis-runtime.py",
          "scripts/analysis-controls.py", "policy/analysis-tools.json", "scripts/tests/test_engine_native.py")
CONTROL_SOURCE = '''//! Fixed healthy, failing and waiting native-supervisor controls.
use std::{env, ffi::OsStr, process::ExitCode, thread, time::Duration};

/// Accept one closed mode and perform only its finite synthetic behavior.
fn main() -> ExitCode {
    let mut arguments = env::args_os().skip(1);
    let selected = arguments.next();
    if arguments.next().is_some() {
        return ExitCode::from(126);
    }
    match selected.as_deref() {
        Some(value) if value == OsStr::new("healthy") => {
            println!("GATE_CONTROL_HEALTHY");
            ExitCode::SUCCESS
        }
        Some(value) if value == OsStr::new("fail") => {
            println!("GATE_CONTROL_FAIL");
            ExitCode::from(7)
        }
        Some(value) if value == OsStr::new("wait") => {
            println!("GATE_CONTROL_WAIT");
            thread::sleep(Duration::from_secs(60));
            ExitCode::SUCCESS
        }
        _ => ExitCode::from(126),
    }
}
'''


def require(condition, message):
    """Reject incomplete admission even under optimized Python."""
    if not condition:
        raise Incomplete(message)


def read_record(path):
    """Reuse bounded descriptor/ancestor custody and strict analyzer JSON admission."""
    return PE.Q.C.read_analysis_json(path, maximum=ALLOCATION.MAX_JSON)


def native_directory(path):
    """Admit one fresh lexical/canonical Windows-owned child without creating it."""
    directory = path.absolute()
    prefix = Path("/mnt/c/Users/perso/AppData/Local/NevroTick")
    require(".." not in directory.parts and directory.is_relative_to(prefix)
            and directory != prefix and not directory.exists(),
            "Native directory must be a fresh Windows NevroTick child.")
    ancestor = directory.parent
    while ancestor != Path("/"):
        require(not ancestor.is_symlink(), "Native directory has a redirected ancestor.")
        ancestor = ancestor.parent
    require(directory.resolve().is_relative_to(prefix.resolve()),
            "Native directory escapes its owned root.")
    return directory


def stable_identity(admitted):
    """Select immutable complete-payload identities, excluding live module objects."""
    return {key: admitted[key] for key in ("stable", "inventories", "readonly_roots",
                                          "policy_sha256", "installation_pointer_sha256")}


def engine_artifact(output, work):
    """Require exactly one Cargo-attributed Windows Engine libtest executable."""
    found = []
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = ALLOCATION.decode(line.encode())
        if (item.get("reason") == "compiler-artifact"
                and item.get("target", {}).get("name") == "nevrotick_engine"
                and item.get("target", {}).get("kind") == ["lib"]
                and item.get("profile", {}).get("test") is True
                and item.get("executable") is not None):
            require(item.get("package_id") == "path+file:///work/fixture/crates/nevrotick-engine#0.0.0",
                    "Engine libtest artifact belongs to an unexpected source package.")
            value = item["executable"]
            require(type(value) is str and value.startswith(f"/work/target/{TARGET}/debug/deps/")
                    and value.endswith(".exe") and ".." not in Path(value).parts,
                    "Unexpected Engine libtest output path.")
            found.append(work / value.removeprefix("/work/"))
    require(len(found) == 1 and found[0].is_file() and not found[0].is_symlink(),
            "Expected one actual emitted Engine libtest executable.")
    return found[0]


def build(args, report):
    """Freeze exact inputs, build offline, inspect emitted PE declarations, and re-admit."""
    require(all(getattr(args, name) is not None for name in
                ("vendor_dir", "toolchain_dir", "xwin_config", "xwin_inventory")),
            "Build requires explicit vendor, toolchain and complete xwin admission paths.")
    require(shutil.disk_usage(args.output_dir).free >= 4 * 1024 ** 3,
            "Build requires four GiB of free output space.")
    require(type(args.jobs) is int and 1 <= args.jobs <= 12, "Build jobs must be within 1–12.")
    policy, inputs = PRODUCT.admit_product(ROOT)
    extras = {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS}
    vendor, toolchain = args.vendor_dir.resolve(strict=True), args.toolchain_dir.resolve(strict=True)
    vendor_before = PRODUCT.admit_vendor(vendor, policy)
    tools_before = PRODUCT.admit_tools(ROOT, toolchain, None, "windows")
    admission = ANALYSIS.admit(ROOT, profile="stable-core")
    require(admission["stable"] == toolchain, "Selected toolchain differs from admitted stable payload.")
    stable_before = stable_identity(admission)
    sdk_before = {str(path): PRODUCT.sha256(path) for path in (args.xwin_config, args.xwin_inventory)}
    report.update(product_source_sha256=inputs, extra_inputs_sha256=extras,
                  candidate_sha256=ALLOCATION.canonical_digest(inputs), tools_sha256=tools_before,
                  stable_payload=stable_before, toolchain=str(toolchain), vendor=str(vendor),
                  xwin_config=str(args.xwin_config.resolve(strict=True)),
                  xwin_inventory=str(args.xwin_inventory.resolve(strict=True)), profile="debug",
                  diagnostic_profile="release", build_jobs=args.jobs, lanes=[])
    source, work = args.output_dir / "source", args.output_dir / "work"
    source.mkdir(mode=0o700); work.mkdir(mode=0o700)
    (work / "fixture").mkdir(mode=0o700)
    (work / "cargo-home").mkdir(mode=0o700)
    for name, checksum in {**inputs, **extras}.items():
        destination = source / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / name, destination)
        require(PRODUCT.sha256(destination) == checksum, "Source changed while freezing.")
    control = source / "gate-control.rs"
    control.write_text(CONTROL_SOURCE)
    report["control_source_sha256"] = PRODUCT.sha256(control)
    libraries, library_inputs = PRODUCT.windows_libraries(
        ROOT, toolchain, args.xwin_config, work, args.xwin_inventory)
    report["windows_inputs"] = {"records_sha256": sdk_before, "library_files": len(library_inputs),
                                "library_map_sha256": ALLOCATION.canonical_digest(library_inputs)}
    (work / "cargo-home/config.toml").write_text(
        '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n')

    def run(label, command):
        """Retain the first raw command outcome before classifying failure."""
        result = ENV.run(PRODUCT.sandbox(toolchain, work, source, vendor, command, None, libraries),
                         cwd=ROOT, timeout=300,
                         environment={"PATH": "/usr/bin:/bin", "NEVROTICK_SYNTHETIC_CREDENTIAL": "synthetic-control"})
        row = dict(result, lane=label, selected_command=command)
        raw_path = args.output_dir / (label + ".json")
        raw_path.write_text(json.dumps(row, indent=2) + "\n")
        report["lanes"].append({"lane": label, "selected_command": command,
                                "exit_code": result["exit_code"], "failure": result["failure"],
                                "raw_path": str(raw_path), "raw_sha256": PRODUCT.sha256(raw_path)})
        return ENV.require_ok(result)

    PRODUCT.boundary_control(run)
    run("runner-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest", "discover",
                            "-s", "/work/fixture/scripts/tests", "-p", "test_engine_native.py", "-v"])
    version = run("compiler-version", ["rustc", "-vV"])
    ENV.require_admitted_compiler(policy=json.loads((ROOT / "policy/tool-manifest.json").read_text()),
                                  channel=tomllib.loads((source / "rust-toolchain.toml").read_text())["toolchain"]["channel"],
                                  compiler_info=version)
    cargo = ["cargo", "--config", f'target.{TARGET}.linker="/work/linker"', "test",
             "--package", "nevrotick-engine", "--lib", "--features", "ke-association-prototype",
             "--frozen", "--jobs", str(args.jobs), "--target", TARGET, "--no-run", "--message-format=json"]
    workspace = tomllib.loads((source / "Cargo.toml").read_text())
    flags = list(ENV.MSVC_FLAGS)
    flags[flags.index("target-cpu=x86-64-v3")] = "target-cpu=x86-64"
    report["product_target_flags"] = ENV.MSVC_FLAGS
    report["diagnostic_target_flags"] = flags
    direct = ["--edition=2024", "--target", TARGET, "-C", "linker=/work/linker",
              *flags, *ALLOCATION.profile_flags("release"), *ALLOCATION.lint_flags(workspace)]
    images = {}
    for name, relative in (("gated-launcher", LAUNCHER), ("cpu-v3-probe", CPU), ("gate-control", "gate-control.rs")):
        source_arg = "/work/fixture/" + relative
        run(name + "-format", ["rustfmt", "--edition", "2024", "--check", source_arg])
        run(name + "-clippy", ["clippy-driver", *direct, "--crate-name", name.replace("-", "_"),
                              "--emit=metadata", "-o", "/work/" + name + ".rmeta", source_arg])
        image = work / (name + ".exe")
        run(name + "-link", ["rustc", *direct, "--crate-name", name.replace("-", "_"),
                            "-o", "/work/" + image.name, source_arg])
        images[name] = image
    # Qualify small diagnostic consumers before the substantial Engine link.
    images["engine-tests"] = engine_artifact(run("engine-libtest-link", cargo), work)
    report["artifacts"] = {}
    inspector = "/toolchain/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-readobj"
    for name, image in images.items():
        raw = PE.pe_readback(image)
        run(name + "-pe", [inspector, "--file-headers", "--coff-load-config", "--coff-debug-directory",
                           "--coff-imports", "/work/" + str(image.relative_to(work))])
        pdb = image.with_suffix(".pdb")
        require(pdb.is_file() and not pdb.is_symlink(), "Matching emitted PDB is absent.")
        for original, destination in ((image, args.output_dir / (name + ".exe")),
                                      (pdb, args.output_dir / (name + ".pdb"))):
            shutil.copyfile(original, destination)
            require(PRODUCT.sha256(destination) == PRODUCT.sha256(original), "Export changed emitted bytes.")
        report["artifacts"][name + ".exe"] = {"path": str(args.output_dir / (name + ".exe")),
                                             "sha256": PRODUCT.sha256(image), "pe": raw,
                                             "pdb_sha256": PRODUCT.sha256(pdb)}
    require(PRODUCT.admit_product(ROOT)[1] == inputs
            and all(PRODUCT.sha256(ROOT / name) == checksum for name, checksum in extras.items())
            and PRODUCT.admit_vendor(vendor, policy) == vendor_before
            and PRODUCT.admit_tools(ROOT, toolchain, None, "windows") == tools_before
            and stable_identity(ANALYSIS.admit(ROOT, profile="stable-core")) == stable_before
            and PRODUCT.library_inventory(libraries) == library_inputs
            and all(PRODUCT.sha256(Path(path)) == checksum for path, checksum in sdk_before.items()),
            "Source, tools, stable payload, vendor or Microsoft inputs changed.")
    report.update(status="passed", bookends_passed=True, native_windows_execution=False)


NATIVE_JOB_OBSERVER = r"""
    public class Member {
        public long pid; public string basename, image_role, read_failure_type; public bool system_console;
    }
    public sealed class Notice : Member { public uint kind; }
    [StructLayout(LayoutKind.Sequential)] struct PortAssociation { public IntPtr key, port; }
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool QueryInformationJobObject(IntPtr job, int kind, IntPtr buffer, uint length, out uint returned);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool IsProcessInJob(IntPtr process, IntPtr job, out bool contained);
    [DllImport("kernel32.dll", SetLastError=true)] static extern IntPtr OpenProcess(uint access, bool inherit, uint pid);
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    static extern bool QueryFullProcessImageNameW(IntPtr process, uint flags, StringBuilder path, ref uint capacity);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern IntPtr CreateIoCompletionPort(IntPtr file, IntPtr existing, UIntPtr key, uint threads);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool SetInformationJobObject(IntPtr job, int kind, ref PortAssociation info, uint length);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool GetQueuedCompletionStatus(IntPtr port, out uint kind, out UIntPtr key, out IntPtr value, uint milliseconds);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool PostQueuedCompletionStatus(IntPtr port, uint kind, UIntPtr key, IntPtr value);
    // Kernel-reported Win32 path, bound to a currently owned-job process handle.
    // Raw paths are never emitted. File hashes are OS provenance, not mapped-image hashes.
    static Member ReadMember(IntPtr job, long pid, string launcher, string child, string console) {
        Member member = new Member {pid=pid}; IntPtr process = IntPtr.Zero;
        try {
            if (pid <= 0 || pid > UInt32.MaxValue) throw new InvalidDataException();
            process = OpenProcess(0x1000, false, (uint)pid); Check(process != IntPtr.Zero);
            bool contained; Check(IsProcessInJob(process, job, out contained));
            if (!contained) throw new InvalidOperationException();
            StringBuilder path = new StringBuilder(32768); uint capacity = 32768;
            Check(QueryFullProcessImageNameW(process, 0, path, ref capacity));
            if (capacity == 0 || capacity >= 32768) throw new InvalidDataException();
            string actual = path.ToString(); member.basename = Path.GetFileName(actual);
            if (String.IsNullOrEmpty(member.basename)) throw new InvalidDataException();
            member.system_console = String.Equals(actual, console, StringComparison.OrdinalIgnoreCase);
            member.image_role = String.Equals(actual, launcher, StringComparison.OrdinalIgnoreCase) ? "launcher" :
                String.Equals(actual, child, StringComparison.OrdinalIgnoreCase) ? "child" :
                member.system_console ? "system-console" : "unknown";
        } catch (Exception exception) { member.read_failure_type = exception.GetType().Name; }
        finally {
            if (process != IntPtr.Zero && !CloseHandle(process)) member.read_failure_type = "Win32Exception";
        }
        return member;
    }
    // One bounded snapshot; counters and membership are separate observations.
    static void ObserveBeforeGate(IntPtr job, Result result, string launcher, string child, string console) {
        IntPtr buffer = IntPtr.Zero;
        try {
            Accounting counts;
            Check(QueryInformationJobObject(job, 1, out counts, (uint)Marshal.SizeOf(typeof(Accounting)), IntPtr.Zero));
            result.total_processes_before_gate = counts.totalProcesses;
            result.active_processes_before_gate = counts.activeProcesses;
            if (IntPtr.Size != 8) throw new InvalidOperationException();
            const int bytes = 8 + 8 * 8; buffer = Marshal.AllocHGlobal(bytes); uint returned;
            Check(QueryInformationJobObject(job, 3, buffer, bytes, out returned));
            if (returned < 8 || returned > bytes) throw new InvalidDataException();
            int assigned = Marshal.ReadInt32(buffer, 0), listed = Marshal.ReadInt32(buffer, 4);
            result.assigned_before_gate = assigned; result.listed_before_gate = listed;
            if (assigned < 1 || assigned > 8 || listed != assigned || returned < 8 + listed * 8)
                throw new InvalidDataException();
            Member[] members = new Member[listed]; result.members_before_gate = members;
            for (int index = 0; index < listed; index++) {
                Member member = ReadMember(job, Marshal.ReadInt64(buffer, 8 + index * 8), launcher, child, console);
                members[index] = member;
                if (member.read_failure_type != null || (member.image_role != "launcher" && member.image_role != "system-console"))
                    throw new InvalidDataException();
            }
        } catch (Exception exception) { result.membership_read_failure_type = exception.GetType().Name; throw; }
        finally { if (buffer != IntPtr.Zero) Marshal.FreeHGlobal(buffer); }
    }
    // Best-effort notifications are supplemental identity evidence. Absence is
    // never inferred: Python requires every lifetime association to have one NEW.
    sealed class JobEvents {
        readonly IntPtr job; readonly string launcher, child, console;
        IntPtr port; Task worker; volatile bool forceStop; int faulted;
        readonly System.Collections.Generic.List<Notice> notices = new System.Collections.Generic.List<Notice>(16);
        bool limitReached; string readFailure;
        public JobEvents(IntPtr ownedJob, string launcherPath, string childPath, string consolePath) {
            job=ownedJob; launcher=launcherPath; child=childPath; console=consolePath;
        }
        public void Start() {
            port = CreateIoCompletionPort(new IntPtr(-1), IntPtr.Zero, UIntPtr.Zero, 1); Check(port != IntPtr.Zero);
            PortAssociation association = new PortAssociation {key=new IntPtr(1), port=port};
            Check(SetInformationJobObject(job, 7, ref association, (uint)Marshal.SizeOf(typeof(PortAssociation))));
            worker = Task.Run(() => Read());
        }
        public void ThrowIfFaulted() {
            if (Volatile.Read(ref faulted) != 0) throw new InvalidOperationException("Owned process identity observation failed.");
        }
        void Read() {
            Stopwatch watch = Stopwatch.StartNew();
            try {
                while (!forceStop && watch.ElapsedMilliseconds < 45000) {
                    uint kind; UIntPtr key; IntPtr value;
                    if (!GetQueuedCompletionStatus(port, out kind, out key, out value, 50)) {
                        int error = Marshal.GetLastWin32Error(); if (error == 258) continue;
                        throw new Win32Exception(error);
                    }
                    if (kind == 0 && key.ToUInt64() == 2 && value == IntPtr.Zero) return;
                    if (key.ToUInt64() != 1) throw new InvalidDataException();
                    Notice notice = new Notice {kind=kind};
                    if (kind == 6 || kind == 7 || kind == 8) notice.pid = value.ToInt64();
                    if (kind == 6) {
                        Member member = ReadMember(job, notice.pid, launcher, child, console);
                        notice.basename=member.basename; notice.image_role=member.image_role;
                        notice.system_console=member.system_console; notice.read_failure_type=member.read_failure_type;
                    }
                    notices.Add(notice);
                    // EXIT is a PID join to historical NEW identity, never a reopen.
                    if ((kind != 4 && kind != 6 && kind != 7 && kind != 8) ||
                        (kind == 6 && (notice.read_failure_type != null || notice.image_role == "unknown")))
                        throw new InvalidDataException();
                    if (notices.Count == 16) { limitReached = true; throw new InvalidDataException(); }
                }
                if (!forceStop) throw new TimeoutException();
            } catch (Exception exception) { readFailure=exception.GetType().Name; Interlocked.Exchange(ref faulted, 1); }
        }
        // Retire all query handles before the caller's final empty-job observation.
        public bool Stop(Result result) {
            try {
                if (worker != null && !worker.IsCompleted) {
                    if (!PostQueuedCompletionStatus(port, 0, new UIntPtr(2), IntPtr.Zero)) {
                        result.event_shutdown_failure_type="Win32Exception"; forceStop=true;
                    }
                    if (!worker.Wait(5000)) {
                        result.event_shutdown_failure_type="TimeoutException"; forceStop=true; return false;
                    }
                }
                result.event_reader_joined=true; result.job_events=notices.ToArray();
                result.event_limit_reached=limitReached; result.event_read_failure_type=readFailure;
                if (port != IntPtr.Zero) {
                    if (!CloseHandle(port)) { result.event_shutdown_failure_type="Win32Exception"; return false; }
                    port=IntPtr.Zero;
                }
                result.event_port_closed=true;
            } catch (Exception exception) { result.event_shutdown_failure_type=exception.GetType().Name; return false; }
            return result.event_shutdown_failure_type == null;
        }
    }
"""



def native_supervisor():
    """Extend the frozen gated observer for fixed children; fail on recipe drift."""
    source = ALLOCATION.NATIVE_SUPERVISOR

    def replace(before, after):
        """Require one exact upstream anchor; never silently alter another lane."""
        nonlocal source
        if source.count(before) != 1:
            raise Incomplete("The reused native-supervisor recipe changed.")
        source = source.replace(before, after)

    source = source.replace("NevroTickAllocationSupervisor", "NevroTickEngineSupervisor")
    replace("No AppVerifier/PageHeap configuration selected for requested-payload counting",
            "No AppVerifier/PageHeap configuration selected for scripted component tests")
    replace("public uint? active_processes_before_handle_release;",
            "public uint? total_processes, total_processes_before_gate, active_processes_before_gate; "
            "public int? assigned_before_gate, listed_before_gate; public Member[] members_before_gate; "
            "public string membership_read_failure_type; public Notice[] job_events; "
            "public bool event_limit_reached, event_reader_joined, event_port_closed, event_job_handle_retained; "
            "public string event_read_failure_type, event_shutdown_failure_type; "
            "public uint? active_processes_before_handle_release;")
    replace("static uint ActiveProcesses(IntPtr job) {", """static uint TotalProcesses(IntPtr job) {
        Accounting accounting;
        Check(QueryInformationJobObject(job, 1, out accounting, (uint)Marshal.SizeOf(typeof(Accounting)), IntPtr.Zero));
        return accounting.totalProcesses;
    }
    static uint ActiveProcesses(IntPtr job) {""")
    replace("    static uint TotalProcesses(IntPtr job) {",
            NATIVE_JOB_OBSERVER + "\n    static uint TotalProcesses(IntPtr job) {")
    replace("public static Result Run(string image, string directory, string[] arguments) {",
            "public static Result Run(string image, string directory, string[] arguments, string gate, string child, string console) {")
    replace("Result result = new Result(); Process process = null; IntPtr job = IntPtr.Zero;",
            "Result result = new Result(); Process process = null; IntPtr job = IntPtr.Zero; JobEvents observer = null;")
    replace("ExtendedLimit limits = new ExtendedLimit(); limits.basic.flags = 0x2000;",
            "ExtendedLimit limits = new ExtendedLimit(); limits.basic.flags = 0x2008; limits.basic.activeLimit = 4;")
    replace("            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(typeof(ExtendedLimit))));",
            "            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(typeof(ExtendedLimit))));\n"
            "            observer = new JobEvents(job, image, child, console); observer.Start();")
    replace("            // The observer has at most the BOM and still needs G; token capture",
            "            ObserveBeforeGate(job, result, image, child, console); observer.ThrowIfFaulted();\n"
            "            // The observer has at most the BOM and still needs G; token capture")
    replace("input.Write('G'); input.Flush(); input.Close(); result.gate_sent = true;", """if (gate == "good") input.Write('G');
            else if (gate == "bad") input.Write('X');
            input.Flush(); input.Close(); result.gate_sent = gate == "good";""")
    replace("            while (!process.WaitForExit(25)) {",
            "            while (!process.WaitForExit(25)) {\n                observer.ThrowIfFaulted();")
    replace("""            process.Dispose(); process = null; result.process_handle_released = true;
            uint remaining;""", """            process.Dispose(); process = null; result.process_handle_released = true;
            if (!observer.Stop(result)) throw new InvalidOperationException("Owned event observer shutdown incomplete.");
            observer.ThrowIfFaulted(); observer = null;
            uint remaining;""")
    replace("result.active_processes_after_cleanup = remaining;",
            "result.active_processes_after_cleanup = remaining; result.total_processes = TotalProcesses(job);")
    replace("""            if (process != null) {
                try {
                    // Failure cleanup closes every owned writer""", """            if (process != null) {
                try {
                    if (assigned) result.active_processes_before_cleanup = ActiveProcesses(job);
                    // Failure cleanup closes every owned writer""")
    replace("""                    if (ActiveProcesses(job) != 0) Check(TerminateJobObject(job, 1));
                    uint remaining;""", """                    if (ActiveProcesses(job) != 0) Check(TerminateJobObject(job, 1));
                    if (observer != null) {
                        if (!observer.Stop(result)) {
                            cleanupOk = false; CleanupFailure(result, new InvalidOperationException("Owned event observer shutdown incomplete."));
                        }
                        if (result.event_reader_joined && result.event_port_closed) observer = null;
                    }
                    uint remaining;""")
    replace("                    if (!CloseHandle(job)) {", """                    if (observer != null && !result.event_reader_joined) {
                        result.event_job_handle_retained = true; cleanupOk = false;
                    } else if (!CloseHandle(job)) {""")
    replace("    $report.runtime_files_sha256 = @{}", """    $systemConsolePath = Join-Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::System)) 'conhost.exe'
    $report.system_console_path = $systemConsolePath
    $report.system_console_provenance = 'Pre/post OS file digest only; mapped-image bytes are not measured.'
    $report.runtime_files_sha256 = @{}""")
    replace("[Diagnostics.Process]::GetCurrentProcess().MainModule.FileName)) {",
            "[Diagnostics.Process]::GetCurrentProcess().MainModule.FileName, $systemConsolePath)) {")
    replace("    $adapterPath = Join-Path $destination 'native-supervisor-adapter.dll'", """    if ($request.mode -cnotin @('cpu','list','ke','control-healthy','control-fail','control-wait') -or
        $request.gate -cnotin @('good','bad','missing')) { throw 'Unadmitted console mode or gate.' }
    $childName = if ($request.mode -ceq 'cpu') { 'cpu-v3-probe.exe' }
        elseif ($request.mode -cin @('list','ke')) { 'engine-tests.exe' } else { 'gate-control.exe' }
    $child = Get-Item -LiteralPath (Join-Path $destination $childName)
    if ($child.PSIsContainer -or (($child.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) {
        throw 'Unadmitted fixed child image.'
    }
    if (@(Get-Process -Name $child.BaseName -ErrorAction SilentlyContinue).Count -ne 0) { throw 'Fixed child basename is already active.' }
    $report.child_image_settings_before = @(Inspect-ImageSettings $child.Name $child.FullName)
    if ($report.child_image_settings_before.Count -ne 0) { throw 'Fixed child has inherited image settings.' }
    $report.child_image_sha256_before = (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($report.child_image_sha256_before -cne $request.child_sha256) { throw 'Fixed child differs from the build.' }
    $memory = Get-CimInstance Win32_OperatingSystem
    $report.available_memory_kib = [long]$memory.FreePhysicalMemory
    if ($report.available_memory_kib -lt 2097152) { throw 'Main Windows available memory is below 2 GiB.' }
    $adapterPath = Join-Path $destination 'native-supervisor-adapter.dll'""")
    replace("""    $arguments = @($request.claims.candidate_sha256, $request.claims.observer_source_sha256,
        $request.claims.build_receipt_sha256, $request.claims.profile)
    foreach ($argument in $arguments[0..2]) { if ($argument -cnotmatch '^[a-f0-9]{64}$') { throw 'Unadmitted native identity claim.' } }
    if ($arguments[3] -cnotin @('debug','release','verify-release')) { throw 'Unadmitted native profile.' }""",
            "    $arguments = @($request.mode)")
    replace("""    try {
        $report.execution = [NevroTickEngineSupervisor]::Run($image.FullName, $destination, $arguments)
        $report.image_sha256_after = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    } finally { $imageLock.Dispose() }
    $report.image_settings_after = @(Inspect-ImageSettings $image.Name $image.FullName)""", """    try {
        $childLock = [IO.File]::Open($child.FullName, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
        try {
            if ((Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -cne $request.image_sha256 -or
                (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -cne $request.child_sha256) {
                throw 'Locked image bytes differ from the frozen build.'
            }
            $report.execution = [NevroTickEngineSupervisor]::Run($image.FullName, $destination, $arguments, $request.gate, $child.FullName, $systemConsolePath)
            $report.image_sha256_after = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
            $report.child_image_sha256_after = (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        } finally { $childLock.Dispose() }
    } finally { $imageLock.Dispose() }
    $report.child_image_settings_after = @(Inspect-ImageSettings $child.Name $child.FullName)
    if ($report.child_image_settings_after.Count -ne 0 -or $report.child_image_sha256_after -cne $report.child_image_sha256_before) {
        throw 'Fixed child image or inherited settings changed.'
    }
    $report.image_settings_after = @(Inspect-ImageSettings $image.Name $image.FullName)""")
    return source


def observed_child_pid(stderr, authored):
    """Join one fixed launcher marker to kernel identity; it grants no identity alone."""
    import re
    require(type(authored) is int and authored in (1, 2), "Unadmitted authored-process count.")
    require(type(stderr) is str, "Native stderr must be bounded text.")
    if authored == 1:
        require(stderr == "launcher-stage=gate;os-error=none\n", "Gate refusal exposed a child marker or another failure.")
        return None
    match = re.fullmatch(r"launcher-child-pid=([1-9][0-9]{0,9})\n", stderr)
    require(match is not None, "Native child needs exactly one bounded PID marker.")
    pid = int(match[1])
    require(pid <= 0xffffffff, "Native child PID exceeds u32.")
    return pid


def validate_processes(value, request, result, authored):
    """Require complete NEW identities, bounded OS helpers and pre-gate exclusion.

    Notification delivery is best-effort. Missing evidence refuses qualification;
    EXIT IDs join historical NEW identities and are never reopened by the collector.
    """
    from pathlib import PureWindowsPath

    def uint(item, maximum=0xffffffff, minimum=0):
        return type(item) is int and minimum <= item <= maximum

    root = result["pid"]
    child = observed_child_pid(result.get("stderr"), authored)
    require(child != root, "Launcher and child PIDs cannot alias.")
    root_name = PureWindowsPath(request["image_path"]).name.casefold()
    child_name = ("cpu-v3-probe.exe" if request["mode"] == "cpu" else
                  "engine-tests.exe" if request["mode"] in ("list", "ke") else "gate-control.exe")
    console_path = value.get("system_console_path")
    require(type(console_path) is str and PureWindowsPath(console_path).is_absolute()
            and PureWindowsPath(console_path).parent.name.casefold() == "system32"
            and PureWindowsPath(console_path).name.casefold() == "conhost.exe"
            and console_path in value["runtime_files_sha256"], "Missing pre/post System32 console-file provenance.")
    for name in ("event_reader_joined", "event_port_closed"):
        require(result.get(name) is True, "Owned event reader or port did not retire.")
    for name in ("event_limit_reached", "event_job_handle_retained"):
        require(result.get(name) is False, "Owned process observation exceeded a bound or retained custody.")
    for name in ("event_read_failure_type", "event_shutdown_failure_type", "membership_read_failure_type"):
        require(result.get(name) is None, "Owned process observation has an unresolved read or shutdown failure.")
    total = result.get("total_processes")
    require(uint(total, 4, authored), "Unexpected lifetime association count.")
    require(uint(result.get("total_processes_before_gate"), 4, 1)
            and result["total_processes_before_gate"] <= total
            and uint(result.get("active_processes_before_gate"), 4, 1)
            and result["active_processes_before_gate"] <= result["total_processes_before_gate"],
            "Invalid pre-gate accounting.")

    def identity(member, *, before_gate=False):
        require(type(member) is dict and uint(member.get("pid"), minimum=1)
                and member.get("read_failure_type") is None, "Unverified owned process identity.")
        pid = member["pid"]
        if pid == root:
            role, basename = "launcher", root_name
        elif child is not None and pid == child and not before_gate:
            role, basename = "child", child_name
        else:
            role, basename = "system-console", "conhost.exe"
        require(type(member.get("basename")) is str and member["basename"].casefold() == basename
                and member.get("image_role") == role
                and member.get("system_console") is (role == "system-console"),
                "Unexpected image path identity or child present before its gate.")
        return pid, role, member["basename"].casefold()

    events = result.get("job_events")
    require(type(events) is list and 1 <= len(events) < 16, "Incomplete or over-limit job notifications.")
    new = {}
    exits = set()
    for event in events:
        require(type(event) is dict and uint(event.get("kind")) and event["kind"] in (4, 6, 7, 8),
                "Unknown job notification.")
        if event["kind"] == 6:
            pid, role, basename = identity(event)
            require(pid not in new, "Duplicate NEW process identity.")
            new[pid] = (role, basename)
        else:
            require(event.get("basename") is None and event.get("image_role") is None
                    and event.get("system_console") is False and event.get("read_failure_type") is None,
                    "EXIT/zero event must not reopen or invent process identity.")
            if event["kind"] == 4:
                require(event.get("pid") == 0 and type(event.get("pid")) is int, "Zero-active event has a PID.")
            else:
                require(uint(event.get("pid"), minimum=1), "Exit event has no bounded PID.")
                pair = (event["kind"], event["pid"])
                require(pair not in exits, "Duplicate exit notification.")
                exits.add(pair)
    require(len(new) == total and root in new and (child is None or child in new)
            and sum(role == "system-console" for role, _ in new.values()) <= authored
            and all(pid in new for _, pid in exits), "Missed, unknown or excessive lifetime process associations.")
    before = result.get("members_before_gate")
    require(type(before) is list and 1 <= len(before) <= 8
            and uint(result.get("assigned_before_gate"), 8, 1)
            and uint(result.get("listed_before_gate"), 8, 1)
            and result["assigned_before_gate"] == result["listed_before_gate"] == len(before),
            "Incomplete pre-gate membership snapshot.")
    seen = set()
    for member in before:
        pid, role, basename = identity(member, before_gate=True)
        require(pid not in seen and new.get(pid) == (role, basename), "Pre-gate identity is duplicate or unaccounted.")
        seen.add(pid)
    require(root in seen, "Launcher absent from pre-gate custody.")



def validate_console(value, request, supervisor_hash, request_hash, *, expected_exit=0,
                     expected_processes=2, timeout_control=False):
    """Classify exact intended controls separately from genuine completed tests."""
    import re
    require(type(value) is dict, "Native supervisor result must be an object.")
    if (type(value.get("schema_version")) is int and value["schema_version"] == 1
            and value.get("status") == "incomplete" and value.get("execution") is None
            and type(value.get("available_memory_kib")) is int and 0 <= value["available_memory_kib"] < 2097152
            and value.get("error") == "Main Windows available memory is below 2 GiB."):
        raise Incomplete("Main Windows available memory is below 2 GiB; no test process started.")
    for name in ("os", "execution"):
        require(type(value.get(name)) is dict, "Native observation has a malformed nested object.")
    expected_status = "completed" if expected_exit == 0 else "incomplete"
    if (type(value.get("schema_version")) is not int or value["schema_version"] != 1
            or value.get("status") != expected_status
            or value.get("registry_writes") is not False
            or value.get("image_settings_before") != [] or value.get("image_settings_after") != []
            or value.get("child_image_settings_before") != [] or value.get("child_image_settings_after") != []
            or value.get("image_sha256_before") != request["image_sha256"]
            or value.get("image_sha256_after") != request["image_sha256"]
            or value.get("child_image_sha256_before") != request["child_sha256"]
            or value.get("child_image_sha256_after") != request["child_sha256"]
            or value.get("supervisor_sha256") != supervisor_hash or value.get("request_sha256") != request_hash
            or value.get("os", {}).get("is_64_bit") is not True
            or type(value.get("available_memory_kib")) is not int or value["available_memory_kib"] < 2097152
            or value.get("runtime_inputs_unchanged") is not True
            or type(value.get("adapter_sha256")) is not str
            or not re.fullmatch("[a-f0-9]{64}", value["adapter_sha256"])
            or type(value.get("runtime_files_sha256")) is not dict or not value["runtime_files_sha256"]
            or any(type(name) is not str or type(digest) is not str or not re.fullmatch("[a-f0-9]{64}", digest)
                   for name, digest in value["runtime_files_sha256"].items())):
        raise Incomplete("Native image, settings, runtime or completed-state admission failed.")
    result = value.get("execution", {})
    if (type(result.get("pid")) is not int or result["pid"] <= 0
            or type(result.get("exit_code")) is not int or result["exit_code"] != expected_exit
            or result.get("cleanup_failure") is not None or result.get("output_overflow") is not False
            or result.get("cleanup_complete") is not True or result.get("streams_released") is not True
            or result.get("process_handle_released") is not True
            or result.get("gate_sent") is not (request["gate"] == "good")
            or result.get("stdin_encoding") != "utf-8" or result.get("stdin_preamble_hex") != "EF-BB-BF"
            or type(result.get("active_processes_before_cleanup")) is not int
            or (not timeout_control and result.get("active_processes_before_cleanup") != 0)
            or (timeout_control and (type(result.get("active_processes_before_cleanup")) is not int
                                     or not 1 <= result["active_processes_before_cleanup"] <= 4))
            or type(result.get("active_processes_after_cleanup")) is not int
            or result["active_processes_after_cleanup"] != 0
            or type(result.get("total_processes")) is not int or not 1 <= result["total_processes"] <= 4):
        raise Incomplete("Owned child failed its exact exit, gate, containment or cleanup contract.")
    failure = result.get("failure")
    if timeout_control:
        if failure != "TimeoutException: Owned child deadline exceeded.":
            raise Incomplete("Waiting control did not reach its intended supervisor timeout.")
    elif failure is not None:
        raise Incomplete("Unexpected supervisor failure in a completed control/test.")
    parent, launcher = result.get("parent_token", {}), result.get("child_token", {})
    for token in (parent, launcher):
        require(type(token) is dict, "Native token observation must be an object.")
        if (not re.fullmatch(r"S-1-\d+(?:-\d+)+", token.get("user_sid", ""))
                or token.get("elevated") is not False or token.get("administrator") is not False
                or type(token.get("elevation_type")) is not int or token["elevation_type"] not in (1, 3)):
            raise Incomplete("Expected ordinary-user supervisor/launcher token was not observed.")
    if parent["user_sid"] != launcher["user_sid"]:
        raise Incomplete("Launcher user differs from the ordinary supervisor.")
    validate_processes(value, request, result, expected_processes)
    return result


def native(args, report):
    """Run fixed controls then the CPU/list/KE chain on an explicitly owned host."""
    import re
    import uuid
    if args.build_receipt is None or args.native_dir is None:
        raise Incomplete("Native lane requires --build-receipt and a fresh --native-dir.")
    build_path = args.build_receipt.absolute()
    build = read_record(build_path)
    require(type(build) is dict, "Native build receipt must be an object.")
    policy, source_before = PRODUCT.admit_product(ROOT)
    extras_before = {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS}
    if (build.get("status") != "passed" or build.get("lane") != "build"
            or build.get("product_source_sha256") != source_before
            or build.get("extra_inputs_sha256") != extras_before):
        raise Incomplete("Native build receipt does not bind the complete current candidate and runner.")
    artifacts = build.get("artifacts", {})
    names = {"engine-tests.exe", "cpu-v3-probe.exe", "gated-launcher.exe", "gate-control.exe"}
    if type(artifacts) is not dict or set(artifacts) != names:
        raise Incomplete("Native build does not contain the four fixed bounded images.")
    for name, item in artifacts.items():
        require(type(item) is dict and type(item.get("path")) is str,
                "Native artifact admission must be a typed record.")
        path = Path(item["path"])
        if (path.is_symlink() or not path.is_file() or path.stat().st_size > 64 * 1024 * 1024
                or path.parent != build_path.parent or PRODUCT.sha256(path) != item.get("sha256")):
            raise Incomplete("Native emitted image is linked, absent, unbounded or changed.")
    directory = native_directory(args.native_dir)
    # The host directory must be an owned fresh LocalAppData child, not UNC/WSL.
    prefix = Path("/mnt/c/Users/perso/AppData/Local/NevroTick")
    if shutil.disk_usage(prefix).free < 2 * 1024**3:
        raise Incomplete("Windows native output needs at least 2 GiB free disk.")
    directory.mkdir(mode=0o700, parents=True)
    powershell = "/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe"
    ps_before = PRODUCT.sha256(Path(powershell))
    report.update(candidate_sha256=build["candidate_sha256"], build_receipt=str(build_path),
                  build_receipt_sha256=PRODUCT.sha256(build_path), windows_directory=str(directory),
                  powershell_sha256=ps_before, product_source_sha256=source_before,
                  extra_inputs_sha256=extras_before, observations=[],
                  token_scope="Supervisor and launcher directly observed; fixed child inherits primary token.",
                  scope="Scripted KE component tests; no real TLS, CNG, UDP, service or clock effects.")
    script = native_supervisor()
    supervisor_hash = hashlib.sha256(script.encode()).hexdigest()
    for label, mode, gate, expected_exit, expected_count in (
            ("missing-gate", "control-healthy", "missing", 126, 1),
            ("wrong-gate", "control-healthy", "bad", 126, 1),
            ("healthy-control", "control-healthy", "good", 0, 2),
            ("failure-control", "control-fail", "good", 7, 2),
            ("timeout-control", "control-wait", "good", 1, 2),
            ("cpu", "cpu", "good", 0, 2),
            ("list", "list", "good", 0, 2),
            ("ke", "ke", "good", 0, 2)):
        observation = directory / label
        observation.mkdir(mode=0o700)
        launcher = observation / ("NevroTick-" + uuid.uuid4().hex + ".exe")
        selected_child = ("cpu-v3-probe.exe" if mode == "cpu" else
                          "engine-tests.exe" if mode in ("list", "ke") else "gate-control.exe")
        for name, destination in (("gated-launcher.exe", launcher),
                                  (selected_child, observation / selected_child)):
            shutil.copyfile(artifacts[name]["path"], destination)
            if PRODUCT.sha256(destination) != artifacts[name]["sha256"]:
                raise Incomplete("Native staging image changed while copying.")
        supervisor = observation / "supervise.ps1"
        supervisor.write_text(script, encoding="utf-8", newline="")
        request = {"image_path": ALLOCATION.windows_path(launcher), "image_sha256": artifacts["gated-launcher.exe"]["sha256"],
                   "child_sha256": artifacts[selected_child]["sha256"], "mode": mode, "gate": gate}
        request_path = observation / "request.json"
        request_path.write_text(json.dumps(request, indent=2) + "\n")
        request_hash = PRODUCT.sha256(request_path)
        raw = ENV.run([powershell, "-NoLogo", "-NoProfile", "-NonInteractive", "-File",
                       ALLOCATION.windows_path(supervisor), "-RequestPath", ALLOCATION.windows_path(request_path)],
                      cwd=observation, timeout=90, environment={"PATH": "/usr/bin:/bin", "WSLENV": ""})
        (args.output_dir / (label + "-invocation.json")).write_text(json.dumps(raw, indent=2) + "\n")
        record = {"label": label, "mode": mode, "gate": gate, "invocation": raw,
                  "raw_path": str(observation / "native-supervision.json")}
        report["observations"].append(record)
        raw_path = observation / "native-supervision.json"
        if raw_path.is_symlink() or not raw_path.is_file() or raw_path.stat().st_size > 1024**2:
            raise Incomplete("Native supervisor produced no bounded regular result.")
        value = read_record(raw_path)
        record["supervision"] = value
        (args.output_dir / (label + "-supervision.json")).write_text(json.dumps(value, indent=2) + "\n")
        expected_parent_exit = 0 if expected_exit == 0 else 1
        if raw["failure"] is not None or raw["exit_code"] != expected_parent_exit:
            raise Incomplete("Native supervisor did not preserve its intended invocation outcome.")
        result = validate_console(value, request, supervisor_hash, request_hash,
                                  expected_exit=expected_exit, expected_processes=expected_count,
                                  timeout_control=label == "timeout-control")
        output, error = result["stdout"], result["stderr"]
        observed_child_pid(error, expected_count)
        expected_output = {"missing-gate": "", "wrong-gate": "",
                           "healthy-control": "GATE_CONTROL_HEALTHY\n",
                           "failure-control": "GATE_CONTROL_FAIL\n",
                           "timeout-control": "GATE_CONTROL_WAIT\n",
                           "cpu": "cpu-os-v3=accepted\n"}.get(label)
        if expected_output is not None and output != expected_output:
            raise Incomplete("Native control/CPU output did not match independent expected facts.")
        if label == "list":
            roster = [line.removesuffix(": test") for line in output.splitlines() if line.endswith(": test")]
            if (len(roster) != 260 or len(set(roster)) != 260
                    or sum(name.startswith("nts_ke_experiment::") for name in roster) != 106):
                raise Incomplete("Windows library/KE test discovery differs from the frozen prediction.")
            report["windows_roster"] = roster
        if label == "ke":
            if not re.search(r"test result: ok\. 106 passed; 0 failed; 0 ignored; 0 measured; 154 filtered out;", output):
                raise Incomplete("Native KE run did not execute exactly its admitted cohort.")
            entered = re.findall(r"^test (nts_ke_experiment::\S+) \.\.\. ok$", output, re.M)
            expected = {name for name in report["windows_roster"] if name.startswith("nts_ke_experiment::")}
            if len(entered) != 106 or set(entered) != expected:
                raise Incomplete("Native KE results do not match the actual discovered identities.")
        record["classification"] = "expected-rejection" if expected_exit else "completed"
    if (PRODUCT.admit_product(ROOT)[1] != source_before
            or {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS} != extras_before
            or PRODUCT.sha256(build_path) != report["build_receipt_sha256"]
            or PRODUCT.sha256(Path(powershell)) != ps_before
            or any(PRODUCT.sha256(Path(item["path"])) != item["sha256"] for item in artifacts.values())):
        raise Incomplete("Native source, images, receipt, supervisor recipe or PowerShell drifted.")
    report["status"] = "passed"


def main():
    """Select one closed lane and retain its partial outcome in fresh owned output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", required=True, choices=("build", "native"))
    parser.add_argument("--jobs", type=int, choices=range(1, 13), default=6,
                        help="Bound Cargo build concurrency; default 6, maximum 12.")
    for name in ("vendor-dir", "toolchain-dir", "xwin-config", "xwin-inventory", "output-dir",
                 "build-receipt", "native-dir"):
        parser.add_argument("--" + name, type=Path, required=name == "output-dir")
    args = parser.parse_args()
    report = {"schema_version": 1, "lane": args.lane, "status": "incomplete", "started_unix": time.time(),
              "scope": "Bounded Windows Engine scripted KE libtest; no product or release qualification."}
    output = None
    try:
        output = ALLOCATION.new_output(args.output_dir)
        args.output_dir = output
        (build if args.lane == "build" else native)(args, report)
    except (Incomplete, PE.Incomplete, ANALYSIS.Incomplete, OSError, ValueError, KeyError, TypeError) as error:
        report["error"] = str(error)
    report["completed_unix"] = time.time()
    if output is not None:
        (output / "verification.json").write_text(json.dumps(report, indent=2, default=str) + "\n")
    print(json.dumps({"status": report["status"], "lane": args.lane,
                      "error": report.get("error"), "verification_path": str(output / "verification.json") if output else None}))
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
