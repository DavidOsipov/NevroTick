#!/usr/bin/env python3
"""Build and run a fail-closed Linux analysis namespace; install nothing.

Callers admit the payload/toolchain contents and select locked Cargo commands.
Only the owned work tree is writable. Rlimits are per-process safeguards, not a
cgroup memory/disk quota; sanitizer mode intentionally has no address-space cap.
"""

from __future__ import annotations

import importlib.util
import json
import math
import os
from pathlib import Path
import re
import stat
import subprocess


SPEC = importlib.util.spec_from_file_location("analysis_environment", Path(__file__).with_name("check-environment.py"))
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
Incomplete = ENV.Incomplete

PUBLIC = ("/usr", "/bin", "/lib", "/lib64", "/etc/alternatives", "/etc/ld.so.cache")
PRIVATE = ("/proc", "/sys", "/dev", "/etc", "/run")
SANITIZER_FLAGS = ("-runs=256", "-max_len=4096", "-timeout=2", "-rss_limit_mb=512",
                   "-malloc_limit_mb=256", "-max_total_time=30")
CORE_SANITIZER_PROFILE = "time-core-s12a"
CORE_SANITIZER_FLAGS = ("-runs=4096", "-max_len=1537", "-timeout=2", "-rss_limit_mb=512",
                        "-malloc_limit_mb=256", "-max_total_time=15")
# Fixed D-only history boundary; the admitted C profile above stays unchanged.
HISTORY_SANITIZER_PROFILE = "time-core-history-s12d"
HISTORY_SANITIZER_FLAGS = ("-runs=4096", "-max_len=1561", "-timeout=2", "-rss_limit_mb=512",
                           "-malloc_limit_mb=256", "-max_total_time=15")
DEFAULT_ADDRESS_SPACE = 8 * 1024 ** 3
# Budget build/test utility work for the owner's twelve logical CPU slots.
MAX_JOBS = 12


def _directory(path: Path, label: str) -> Path:
    path = Path(path)
    if not path.is_absolute() or path != path.resolve(strict=True) or not path.is_dir():
        raise Incomplete(f"{label} must be an existing absolute directory without symlink redirection.")
    return path


def _overlap(left: Path, right: Path) -> bool:
    return left.is_relative_to(right) or right.is_relative_to(left)


def _narrow(path: Path) -> None:
    if (len(path.parts) < 3 or path.is_mount()
            or (path.parent == Path("/mnt") and re.fullmatch(r"[A-Za-z]", path.name))
            or path == Path.home().resolve() or path.parent == Path("/home")
            or path == Path("/work") or any(path.is_relative_to(Path(item)) for item in PRIVATE)):
        raise Incomplete("Broad system/home mounts and reserved namespace paths are forbidden.")


def _owned_work(path: Path, root: Path) -> None:
    _narrow(path)
    if _overlap(path, root):
        raise Incomplete("Writable work must be outside and separate from the checkout.")
    if path.stat().st_uid != os.geteuid() or stat.S_IMODE(path.stat().st_mode) & 0o077:
        raise Incomplete("Work must be owned by this user with private directory permissions.")

    def unreadable(error):
        raise Incomplete("Work could not be completely inspected.") from error

    aliases = {}
    for directory, subdirs, files in os.walk(path, followlinks=False, onerror=unreadable):
        for name in subdirs + files:
            item = Path(directory) / name
            details = item.lstat()
            if details.st_uid != os.geteuid():
                raise Incomplete("Work contains an unowned entry.")
            if stat.S_ISDIR(details.st_mode):
                continue
            if stat.S_ISLNK(details.st_mode):
                try:
                    target = item.resolve(strict=True)
                except (OSError, RuntimeError) as error:
                    raise Incomplete("Work contains a broken or cyclic symlink.") from error
                if not target.is_relative_to(path) or not target.is_file():
                    raise Incomplete("Work symlinks must resolve to internal regular files; directory links are forbidden.")
            elif not stat.S_ISREG(details.st_mode):
                raise Incomplete("Work contains a special-file entry.")
            identity = (details.st_dev, details.st_ino)
            count, links = aliases.get(identity, (0, details.st_nlink))
            if links != details.st_nlink:
                raise Incomplete("Work hardlink identity changed during inspection.")
            aliases[identity] = (count + 1, links)
    if any(count != links for count, links in aliases.values()):
        raise Incomplete("Work contains a hardlink with an alias outside owned work.")


def _sealed_inputs(work: Path, paths) -> list[Path]:
    """Admit immutable input islands without a writable alias in owned work."""
    sources = []
    for value in paths:
        source = Path(value)
        if (not source.is_absolute() or source == work or not source.is_relative_to(work)
                or source != source.resolve(strict=True)):
            raise Incomplete("Sealed inputs must be canonical strict descendants of owned work.")
        if any(_overlap(source, work / name) for name in ("home", "tmp", "target")):
            raise Incomplete("Reserved writable home/temp/target paths cannot be sealed.")
        if not source.is_dir() and not source.is_file():
            raise Incomplete("Sealed inputs must be regular files or directories.")
        if any(_overlap(source, other) for other in sources):
            raise Incomplete("Duplicate or overlapping sealed inputs are forbidden.")
        sources.append(source)

    aliases = {}

    def inspect(item):
        details = item.lstat()
        if stat.S_ISLNK(details.st_mode):
            raise Incomplete("Sealed input symlinks are forbidden.")
        if stat.S_ISDIR(details.st_mode):
            with os.scandir(item) as children:
                for child in children:
                    inspect(Path(child.path))
        elif stat.S_ISREG(details.st_mode):
            identity = (details.st_dev, details.st_ino)
            count, links = aliases.get(identity, (0, details.st_nlink))
            if links != details.st_nlink:
                raise Incomplete("Sealed hardlink identity changed during inspection.")
            aliases[identity] = (count + 1, links)
        else:
            raise Incomplete("Special files are forbidden in sealed inputs.")

    for source in sources:
        inspect(source)
    if any(count != links for count, links in aliases.values()):
        raise Incomplete("Sealed inputs have a hardlink alias outside the sealed set.")
    return sources


def _helpers(root: Path) -> dict[str, str]:
    policy = json.loads((root / "policy/tool-manifest.json").read_text(encoding="utf-8"))
    admitted = {}
    for name in ("bwrap", "prlimit", "cc", "ld", "python3"):
        entry = policy["bootstrap_helpers"][name]
        path = Path(entry["path"]).resolve(strict=True)
        if not path.is_file() or not os.access(path, os.X_OK) or not path.is_relative_to(Path("/usr")):
            raise Incomplete(f"Missing public bootstrap helper: {name}; host fallback is forbidden.")
        ENV.require_admitted_binary(path, entry["sha256"], name)
        admitted[name] = str(path)
    return admitted


def _sanitizer(command: list[str], profile="legacy") -> None:
    """Select one fixed reviewed profile; callers cannot supply arbitrary limits."""
    if profile == "legacy":
        flags = SANITIZER_FLAGS
    elif profile == CORE_SANITIZER_PROFILE:
        flags = CORE_SANITIZER_FLAGS
    elif profile == HISTORY_SANITIZER_PROFILE:
        flags = HISTORY_SANITIZER_FLAGS
    else:
        raise Incomplete("Unknown fixed sanitizer profile.")
    for required in flags:
        key = required.split("=", 1)[0] + "="
        if [item for item in command if item.startswith(key)] != [required]:
            raise Incomplete("Sanitizer mode requires the fixed bounded libFuzzer flags exactly once.")
    if any(item.startswith(("-fork=", "-jobs=", "-workers=", "-merge=", "-minimize_crash=",
                            "-ignore_", "-handle_", "-error_exitcode=", "-timeout_exitcode="))
           for item in command):
        raise Incomplete("Sanitizer subprocess campaigns and failure-handling overrides are forbidden.")
    if profile in {CORE_SANITIZER_PROFILE, HISTORY_SANITIZER_PROFILE}:
        permitted = {*flags, "-artifact_prefix=/work/fuzz-artifacts/"}
        if any(item.startswith("-") and item not in permitted for item in command[1:]):
            raise Incomplete("Core sanitizer accepts only its exact reviewed flags.")


def sandbox_command(root: Path, work: Path, command: list[str], *, readonly=(), sealed=(), path_dirs=(),
                    environment=None, cwd: Path | None = None, mode="ordinary", cpu_seconds=120,
                    address_space_bytes=DEFAULT_ADDRESS_SPACE, build_jobs=MAX_JOBS,
                    sanitizer_profile="legacy", cargo_target_mode="shared", process_limit=128) -> list[str]:
    """Return argv with admitted helpers, absolute readonly binds and `/work`.

    `sealed` protects canonical input files/directories strictly inside work;
    outputs stay writable. `environment` contains only caller-reviewed values;
    ambient values are never copied. Reserved work/offline/PATH settings cannot
    be overridden. Callers admit input contents and executable payload hashes.
    `build_jobs` bounds inner Cargo work; outer mutation workers select one.
    `cargo_target_mode` preserves the shared target by default; `worker` omits
    that setting so an admitted worker uses its own Cargo target configuration.
    `process_limit=512` is reserved for a caller-admitted mutation campaign:
    ordinary resource mode, worker-owned targets and one inner Cargo job.
    All other invocations retain the 128-task default; neither profile changes
    namespace isolation, memory/file/CPU limits or any host process limit.
    """
    if type(cargo_target_mode) is not str or cargo_target_mode not in ("shared", "worker"):
        raise Incomplete("Unknown Cargo target mode.")
    if os.geteuid() == 0:
        raise Incomplete("Run analysis as an ordinary user, never root.")
    root, work = _directory(root, "Checkout"), _directory(work, "Work")
    _owned_work(work, root)
    sources = [_directory(path, "Readonly input") for path in readonly]
    for source in sources:
        _narrow(source)
        if _overlap(source, work):
            raise Incomplete("Readonly source and writable work cannot overlap.")
    if len(set(sources)) != len(sources):
        raise Incomplete("Duplicate readonly inputs are forbidden.")
    sealed_sources = _sealed_inputs(work, sealed)
    directories = [_directory(path, "Tool PATH") for path in path_dirs]
    if any(not any(path.is_relative_to(source) for source in sources) for path in directories):
        raise Incomplete("Tool PATH directories must belong to admitted readonly inputs.")
    cwd = _directory(cwd or work, "Working directory")
    if not cwd.is_relative_to(work) and not any(cwd.is_relative_to(source) for source in sources):
        raise Incomplete("Working directory is outside admitted inputs/work.")
    if (not isinstance(command, list) or not command
            or any(not isinstance(item, str) or not item or "\0" in item for item in command)):
        raise Incomplete("Analysis command must be a nonempty literal argv list.")
    if mode not in ("ordinary", "sanitizer"):
        raise Incomplete("Unknown analysis resource mode.")
    if type(build_jobs) is not int or not 1 <= build_jobs <= MAX_JOBS:
        raise Incomplete("Build jobs must be an integer between 1 and 12.")
    if type(process_limit) is not int or process_limit not in (128, 512):
        raise Incomplete("Process limit must be exactly 128 or 512 tasks.")
    if process_limit == 512 and (mode != "ordinary" or cargo_target_mode != "worker" or build_jobs != 1):
        raise Incomplete("The 512-task profile requires ordinary mode, worker-owned targets and one inner job.")
    if type(cpu_seconds) is not int or not 1 <= cpu_seconds <= 3600:
        raise Incomplete("CPU budget must be an integer between 1 and 3600 seconds.")
    if type(address_space_bytes) is not int or not 256 * 1024 ** 2 <= address_space_bytes <= 64 * 1024 ** 3:
        raise Incomplete("Ordinary address-space limit is outside the admitted range.")
    if mode == "sanitizer":
        _sanitizer(command, sanitizer_profile)
    elif sanitizer_profile != "legacy":
        raise Incomplete("A sanitizer profile cannot change ordinary resource policy.")
    helpers = _helpers(root)
    for name in ("home", "tmp", "cargo-home", "target"):
        (work / name).mkdir(mode=0o700, exist_ok=True)
    trusted = {"PATH": ":".join([*(str(path) for path in directories), "/usr/bin", "/bin"]),
               "LANG": "C.UTF-8", "HOME": "/work/home", "TMPDIR": "/work/tmp",
               "TMP": "/work/tmp", "TEMP": "/work/tmp", "CARGO_HOME": "/work/cargo-home",
               "CARGO_TARGET_DIR": "/work/target", "CARGO_NET_OFFLINE": "true",
               "CARGO_INCREMENTAL": "0", "CARGO_BUILD_JOBS": str(build_jobs), "CARGO_TERM_COLOR": "never",
               "RUSTUP_AUTO_INSTALL": "0"}
    if cargo_target_mode == "worker":
        del trusted["CARGO_TARGET_DIR"]
    for key, value in (environment or {}).items():
        if (not isinstance(key, str) or not re.fullmatch(r"[A-Z_][A-Z_0-9]*", key)
                or not isinstance(value, str) or "\0" in value or key in trusted
                or key in {"CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR",
                           "LD_PRELOAD", "LD_AUDIT", "LD_LIBRARY_PATH"}):
            raise Incomplete("Invalid or reserved explicit analysis environment setting.")
        trusted[key] = value
    args = [helpers["bwrap"], "--unshare-all", "--die-with-parent", "--new-session",
            "--cap-drop", "ALL", "--clearenv", "--bind", str(work), "/work",
            "--bind", str(work / "tmp"), "/tmp"]
    for item in PUBLIC:
        if Path(item).exists():
            args.extend(["--ro-bind", item, item])
    for source in sources:
        args.extend(["--ro-bind", str(source), str(source)])
    for source in sealed_sources:
        destination = Path("/work") / source.relative_to(work)
        args.extend(["--ro-bind", str(source), str(destination)])
    namespace_cwd = Path("/work") / cwd.relative_to(work) if cwd.is_relative_to(work) else cwd
    args.extend(["--proc", "/proc", "--dev", "/dev", "--chdir", str(namespace_cwd)])
    for key, value in trusted.items():
        args.extend(["--setenv", key, value])
    # Install the selected bounded NPROC inside the new namespace, not on the host.
    limits = [f"--cpu={cpu_seconds}", f"--nproc={process_limit}", "--nofile=256", "--fsize=268435456"]
    if mode == "ordinary":
        limits.insert(0, f"--as={address_space_bytes}")
    return args + ["--", helpers["prlimit"], *limits, "--", *command]


def run(root: Path, work: Path, command: list[str], *, timeout=180, **options) -> dict:
    """Run with ENV.run's wall/output/process-group custody; preserve exit codes."""
    if not isinstance(timeout, (int, float)) or not math.isfinite(timeout) or not 0 < timeout <= 3600:
        raise Incomplete("Wall budget must be finite, positive and at most 3600 seconds.")
    argv = sandbox_command(root, work, command, **options)
    return ENV.run(argv, cwd=Path(root), timeout=timeout,
                   environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
                   stdin=subprocess.DEVNULL)
