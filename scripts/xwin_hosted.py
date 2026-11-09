"""Fixed Linux hosted acquisition boundary for setup-xwin; no mount creation."""

from contextlib import contextmanager
from dataclasses import dataclass
import fcntl
import gzip
import hashlib
import importlib.util
import io
import json
import math
import os
from pathlib import Path
import re
import resource
import stat
import subprocess
import sys
import tarfile
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
PAYLOAD_BYTES = 2 * 1024 ** 3
PAYLOAD_INODES = 65_536
MAX_EXPANDED = 64 * 1024 ** 2
MAX_INVENTORY = 16 * 1024 ** 2
WHOLE_SECONDS = 600
MAX_OUTPUT = 2 * 1024 ** 2


@dataclass(frozen=True)
class HostedLayout:
    """Canonical fixed destinations; construction alone supplies no admission."""
    profile: Path
    payload: Path
    bootstrap: Path
    cache: Path
    receipt: Path
    locator: Path


def _require(value, message):
    if not value:
        raise RuntimeError(message)


def _absent(path):
    try:
        path.lstat()
    except FileNotFoundError:
        return True
    return False


def _directory(path):
    try:
        info = path.lstat()
    except OSError as error:
        raise RuntimeError("Owned directory is unavailable") from error
    _require(stat.S_ISDIR(info.st_mode) and info.st_uid == os.geteuid()
             and stat.S_IMODE(info.st_mode) == 0o700, "Owned directory requires ordinary UID and mode700")
    return info


def read_mountinfo():
    """Read a finite kernel mount table; callers cannot choose another input."""
    with open('/proc/self/mountinfo', 'rb') as stream:
        data = stream.read(1024 * 1024 + 1)
    _require(len(data) <= 1024 * 1024, "Mount table exceeds admission budget")
    return data.decode('utf-8', errors='strict')


def _unescape(value):
    escapes = {'040': ' ', '011': '\t', '012': '\n', '134': '\\'}
    _require(not re.search(r'\\(?!040|011|012|134)', value), "Unknown mount path escape")
    return re.sub(r'\\(040|011|012|134)', lambda match: escapes[match[1]], value)


def _mounts():
    rows = []
    for line in read_mountinfo().splitlines():
        parts = line.split(' - ')
        _require(len(parts) == 2, "Malformed mount table row")
        left, right = parts[0].split(), parts[1].split()
        _require(len(left) >= 6 and len(right) >= 3 and re.fullmatch(r'\d+:\d+', left[2]), "Malformed mount identity")
        rows.append({'mount_id': int(left[0]), 'device': left[2], 'root': _unescape(left[3]),
                     'point': Path(_unescape(left[4])), 'options': set(left[5].split(',')),
                     'optional': left[6:], 'fstype': right[0]})
    _require(0 < len(rows) <= 65_536, "Mount count outside admission budget")
    return rows


def quota_facts(layout, *, worker=False):
    """Observe total quota, not remaining free space; reject unowned aliases."""
    rows = _mounts()
    selected = [row for row in rows if row['point'] == layout.payload]
    _require(len(selected) == 1, "Payload must be one exact dedicated mount")
    mount = selected[0]
    _require(mount['fstype'] == 'tmpfs' and mount['root'] == '/'
             and {'rw', 'noexec', 'nodev', 'nosuid'} <= mount['options']
             and not mount['optional'], "Payload mount type/options/propagation differ")
    info = layout.payload.stat()
    device = f'{os.major(info.st_dev)}:{os.minor(info.st_dev)}'
    _require(device == mount['device'] and info.st_dev != layout.profile.stat().st_dev,
             "Payload is not a distinct observed filesystem")
    for row in rows:
        if row is mount:
            continue
        _require(layout.payload not in row['point'].parents, "Nested payload mount is forbidden")
        if row['device'] == device:
            _require(worker and row['point'] == Path('/tmp') and row['root'] == '/tmp'
                     and {'rw', 'noexec', 'nodev', 'nosuid'} <= row['options'], "Payload mount alias is forbidden")
    usage = os.statvfs(layout.payload)
    _require(usage.f_frsize > 0 and usage.f_blocks * usage.f_frsize == PAYLOAD_BYTES
             and usage.f_files == PAYLOAD_INODES, "Payload requires total2GiB and65536inodes")
    return {'mount_id': mount['mount_id'], 'device': device, 'fstype': mount['fstype'],
            'options': sorted(mount['options']), 'total_bytes': PAYLOAD_BYTES, 'total_inodes': PAYLOAD_INODES}


def admit_layout(profile, receipt, environ=None, *, worker=False):
    """Refuse before acquisition; environment markers declare scope, not trust."""
    environment = os.environ if environ is None else environ
    _require(sys.platform == 'linux' and os.geteuid() != 0, "Hosted acquisition requires ordinary Linux UID")
    _require(all(environment.get(key) == value for key, value in {
        'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted', 'RUNNER_OS': 'Linux'}.items()),
        "Explicit hosted Linux markers required")
    _require(receipt is not None and isinstance(profile, Path) and isinstance(receipt, Path), "New explicit receipt required")
    runner_temp = Path(environment.get('RUNNER_TEMP', ''))
    _require(runner_temp.is_absolute() and runner_temp.resolve(strict=True) == runner_temp
             and profile.is_absolute() and profile.resolve(strict=True) == profile
             and profile.parent == runner_temp and profile != ROOT and ROOT not in profile.parents,
             "Use an exact canonical external RUNNER_TEMP child")
    payload = profile / 'payload'
    _directory(profile); _directory(payload)
    expected_children = {'home', 'tmp'} if worker else set()
    observed_children = set()
    with os.scandir(payload) as children:
        for child in children:
            _require(child.name in expected_children, "Pre-existing payload cannot be adopted")
            _directory(payload / child.name)
            with os.scandir(payload / child.name) as contents:
                _require(next(contents, None) is None, "Worker home/tmp must be new and empty")
            observed_children.add(child.name)
    _require(observed_children == expected_children, "Worker preparation directories differ")
    _require(receipt.is_absolute() and receipt.resolve(strict=False) == receipt
             and payload in receipt.parents and _absent(receipt)
             and receipt != payload / 'xwin.json' and receipt != payload / 'inventory.json', "Receipt destination is not new and below payload")
    layout = HostedLayout(profile, payload, profile / 'bootstrap', payload / 'cache', receipt, payload / 'xwin.json')
    _require(receipt != layout.cache and layout.cache not in receipt.parents,
             "Receipt must stay outside the cache inventory it describes")
    _require(_absent(layout.cache) and _absent(layout.locator), "Existing cache/locator cannot be adopted")
    quota_facts(layout, worker=worker)
    return layout


@contextmanager
def ownership(layout):
    """Hold only this profile's regular private lock, without unlinking it."""
    try:
        fd = os.open(layout.profile / '.owner.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC, 0o600)
        info = os.fstat(fd)
        _require(stat.S_ISREG(info.st_mode) and info.st_uid == os.geteuid() and info.st_nlink == 1
                 and stat.S_IMODE(info.st_mode) == 0o600 and info.st_size <= 4096, "Invalid profile ownership lock")
        try:
            fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise RuntimeError("Profile already has an acquisition owner") from error
        yield
    except OSError as error:
        raise RuntimeError("Profile ownership admission failed") from error
    finally:
        if 'fd' in locals():
            os.close(fd)


def _stamp(info):
    return info.st_dev, info.st_ino, info.st_mode, info.st_nlink, info.st_size, info.st_mtime_ns, info.st_ctime_ns


def _metadata(root, max_entries, max_bytes, max_file_bytes, max_depth, directory_fd=None):
    _require(root.is_absolute() and root.resolve(strict=True) == root, "Inventory root must be canonical")
    root_fd = (os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
               if directory_fd is None else os.dup(directory_fd))
    try:
        _require(stat.S_ISDIR(os.fstat(root_fd).st_mode) and _stamp(os.fstat(root_fd)) == _stamp(root.lstat()),
                 "Held inventory directory differs from the canonical root")
    except BaseException:
        os.close(root_fd)
        raise
    files, directories, total, count = {}, {'': _stamp(os.fstat(root_fd))}, 0, 0
    def walk(fd, prefix, depth):
        nonlocal total, count
        with os.scandir(fd) as entries:
            for entry in entries:
                count += 1
                name = entry.name
                relative = prefix + '/' + name if prefix else name
                _require(count <= max_entries and depth <= max_depth and len(relative.encode()) <= 1024
                         and len(name.encode()) <= 255, "Inventory count/depth/path budget exceeded")
                info = os.stat(name, dir_fd=fd, follow_symlinks=False)
                _require(info.st_dev == os.fstat(root_fd).st_dev, "Inventory crossed a filesystem boundary")
                if stat.S_ISDIR(info.st_mode):
                    child = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=fd)
                    try:
                        _require(_stamp(os.fstat(child)) == _stamp(info), "Directory changed during admission")
                        directories[relative] = _stamp(info)
                        walk(child, relative, depth + 1)
                    finally:
                        os.close(child)
                else:
                    _require(stat.S_ISREG(info.st_mode) and info.st_nlink == 1
                             and 0 <= info.st_size <= max_file_bytes, "Linked/special/oversized inventory file")
                    total += info.st_size
                    _require(total <= max_bytes, "Inventory aggregate byte budget exceeded")
                    files[relative] = _stamp(info)
    try:
        walk(root_fd, '', 1)
        return root_fd, files, directories
    except BaseException:
        os.close(root_fd)
        raise


def inventory(root, *, max_entries=65_536, max_bytes=PAYLOAD_BYTES, max_file_bytes=PAYLOAD_BYTES, max_depth=16, directory_fd=None):
    """Admit the complete finite metadata tree before hashing any file content."""
    _require(all(type(n) is int and n > 0 for n in [max_entries, max_bytes, max_file_bytes, max_depth]), "Invalid inventory budgets")
    root_fd, files, directories = _metadata(root, max_entries, max_bytes, max_file_bytes, max_depth, directory_fd)
    result = {}
    def parent_fd(relative):
        fd = os.dup(root_fd)
        try:
            current = ''
            for component in relative.split('/')[:-1]:
                current = current + '/' + component if current else component
                child = os.open(component, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=fd)
                os.close(fd); fd = child
                _require(_stamp(os.fstat(fd)) == directories[current], "Inventory ancestor changed")
            return fd
        except BaseException:
            os.close(fd)
            raise
    try:
        for relative, expected in sorted(files.items()):
            parent = parent_fd(relative)
            fd = None
            try:
                fd = os.open(relative.split('/')[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC, dir_fd=parent)
                _require(_stamp(os.fstat(fd)) == expected, "Inventory file changed before hashing")
                size, digest = 0, hashlib.sha256()
                while size < expected[4]:
                    chunk = os.read(fd, min(65536, expected[4] - size))
                    _require(chunk, "Inventory file ended early")
                    size += len(chunk); digest.update(chunk)
                _require(os.read(fd, 1) == b'' and _stamp(os.fstat(fd)) == expected, "Inventory file grew/changed")
                result[relative] = {'bytes': size, 'sha256': digest.hexdigest()}
            finally:
                if fd is not None: os.close(fd)
                os.close(parent)
        check_fd, after_files, after_dirs = _metadata(root, max_entries, max_bytes, max_file_bytes, max_depth, root_fd)
        os.close(check_fd)
        _require(after_files == files and after_dirs == directories, "Inventory changed before handoff")
        return result
    finally:
        os.close(root_fd)


def tar_members(blob, version):
    """Return only four exact bounded regular members from the pinned tool tar."""
    _require(len(blob) <= 32 * 1024 ** 2 and re.fullmatch(r'\d+\.\d+\.\d+', version), "Invalid bootstrap archive/version")
    with gzip.GzipFile(fileobj=io.BytesIO(blob)) as stream:
        expanded = stream.read(MAX_EXPANDED + 1)
    _require(len(expanded) <= MAX_EXPANDED, "Bootstrap expanded-byte budget exceeded")
    prefix = f'xwin-{version}-x86_64-unknown-linux-musl/'
    allowed = {'xwin', 'LICENSE-APACHE', 'LICENSE-MIT', 'README.md'}
    result, seen, count = {}, set(), 0
    with tarfile.open(fileobj=io.BytesIO(expanded), mode='r:') as bundle:
        for member in bundle:
            count += 1
            _require(count <= 16 and member.name not in seen and not member.pax_headers,
                     "Duplicate/extended/over-count bootstrap member")
            seen.add(member.name)
            if member.isdir():
                _require(member.name in {prefix, prefix.rstrip('/')} and not any(name.rstrip('/') == prefix.rstrip('/') for name in seen - {member.name}), "Unknown/aliased bootstrap directory")
                continue
            _require(member.name.startswith(prefix) and member.name[len(prefix):] in allowed
                     and member.type in {tarfile.REGTYPE, tarfile.AREGTYPE}, "Unknown/linked bootstrap member")
            name = member.name[len(prefix):]
            limit = 32 * 1024 ** 2 if name == 'xwin' else 1024 ** 2
            _require(0 < member.size <= limit and name not in result, "Bootstrap member size/identity differs")
            stream = bundle.extractfile(member)
            _require(stream is not None, "Missing bootstrap member extent")
            data = stream.read(limit + 1)
            _require(len(data) == member.size, "Bootstrap member grew/ended early")
            result[name] = data
    _require(set(result) == allowed, "Incomplete bootstrap tool/grant inventory")
    return result


def run_checked(command, *, cwd, environment, timeout, max_output=MAX_OUTPUT, runner):
    """Use the shared owned-process supervisor; never consume failed output."""
    _require(0 < timeout <= WHOLE_SECONDS and 0 < max_output <= MAX_OUTPUT, "Invalid process budget")
    result = runner(command, cwd=cwd, environment=environment, timeout=timeout,
                    stdin=subprocess.DEVNULL, max_output=max_output)
    _require(result.get('failure') is None and result.get('exit_code') == 0,
             f"Bounded acquisition process failed: {result.get('failure') or 'nonzero exit'} ({result.get('exit_code')})")
    return result['output']


def write_json(path, value, maximum, *, replace=False):
    """Publish one bounded owned record; caller supplies its admitted destination."""
    _require(replace or _absent(path), "Record destination already exists")
    path.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    _require(path.parent.resolve(strict=True) == path.parent, "Record ancestor is linked")
    fd, name = tempfile.mkstemp(prefix='.xwin-record-', dir=path.parent)
    temporary = Path(name)
    try:
        size, digest = 0, hashlib.sha256()
        with os.fdopen(fd, 'wb') as stream:
            for chunk in json.JSONEncoder(separators=(',', ':'), sort_keys=True, allow_nan=False).iterencode(value):
                data = chunk.encode(); size += len(data)
                _require(size <= maximum, "Record byte budget exceeded")
                stream.write(data); digest.update(data)
        if replace:
            _require(path.is_file() and not path.is_symlink(), "Cannot replace an unowned record")
            os.replace(temporary, path)
        else:
            os.link(temporary, path)
        return {'path': str(path), 'bytes': size, 'sha256': digest.hexdigest()}
    finally:
        temporary.unlink(missing_ok=True)


def process_module(root):
    spec = importlib.util.spec_from_file_location('xwin_owned_process', root / 'scripts/check-environment.py')
    _require(spec is not None and spec.loader is not None, "Shared process supervisor unavailable")
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module


def read_record(root, path, maximum):
    """Reuse the acquisition owner's descriptor-bounded regular-file reader."""
    spec = importlib.util.spec_from_file_location('xwin_record_reader', root / 'scripts/bounded_download.py')
    _require(spec is not None and spec.loader is not None, "Shared record reader unavailable")
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module.bounded_file_bytes(path, maximum)


def namespace(root, layout, *, worker, environment):
    """Expose only fixed source/tool/OS inputs; native xwin cannot write bootstrap."""
    command = ['/usr/bin/bwrap', '--unshare-all', '--unshare-user', '--share-net', '--die-with-parent', '--new-session',
               '--cap-drop', 'ALL', '--clearenv', '--proc', '/proc', '--dev', '/dev', '--remount-ro', '/dev',
               '--bind', str(layout.payload / 'tmp'), '/tmp', '--perms', '0700', '--dir', str(layout.profile),
               '--bind', str(layout.payload), str(layout.payload)]
    for path in ['/usr', '/bin', '/lib', '/lib64', '/etc/alternatives', '/etc/ld.so.cache', '/etc/ssl',
                 '/etc/resolv.conf', '/etc/hosts', '/etc/nsswitch.conf', '/etc/gai.conf']:
        if Path(path).exists(): command += ['--ro-bind', path, path]
    command += ['--bind' if worker else '--ro-bind', str(layout.bootstrap), str(layout.bootstrap)]
    if worker:
        for relative in ['scripts/setup-xwin.py', 'scripts/xwin_hosted.py', 'scripts/bounded_download.py',
                         'scripts/check-environment.py', 'policy/tool-manifest.json']:
            command += ['--ro-bind', str(root / relative), str(root / relative)]
    for name, value in environment.items():
        command += ['--setenv', name, value]
    command += ['--remount-ro', '/']
    if not worker:
        command += ['--disable-userns', '--assert-userns-disabled']
    command += ['--chdir', str(layout.payload), '--', '/usr/bin/prlimit', '--as=1073741824', '--cpu=600',
                '--nproc=128', '--nofile=256', '--fsize=2147483648', '--']
    return command


def worker_environment(layout, deadline):
    return {'PATH': '/usr/bin:/bin', 'HOME': str(layout.payload / 'home'), 'TMPDIR': str(layout.payload / 'tmp'),
            'LANG': 'C.UTF-8', 'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted',
            'RUNNER_OS': 'Linux', 'RUNNER_TEMP': str(layout.profile.parent),
            'NEVROTICK_XWIN_WORKER': '1', 'NEVROTICK_XWIN_DEADLINE': str(deadline)}


def remaining_deadline(environment):
    _require(environment.get('NEVROTICK_XWIN_WORKER') == '1', "Worker requires its owned outer supervisor")
    value = float(environment.get('NEVROTICK_XWIN_DEADLINE', 'nan'))
    remaining = value - time.monotonic()
    _require(math.isfinite(value) and 0 < remaining <= WHOLE_SECONDS, "Worker deadline expired or differs")
    return remaining


def require_worker_boundary():
    """Check actual private-worker containment before acquisition; not caller authentication."""
    _require(sys.platform == 'linux' and 0 < os.getuid() == os.geteuid(), "Worker requires equal ordinary UIDs")
    for kind, maximum in [(resource.RLIMIT_AS, 1024 ** 3), (resource.RLIMIT_CPU, WHOLE_SECONDS),
                          (resource.RLIMIT_NPROC, 128), (resource.RLIMIT_NOFILE, 256),
                          (resource.RLIMIT_FSIZE, PAYLOAD_BYTES)]:
        _require(all(0 < value <= maximum for value in resource.getrlimit(kind)),
                 "Worker requires finite admitted soft/hard resource limits")
    try:
        _require(os.statvfs('/').f_flag & os.ST_RDONLY, "Worker root filesystem must be read-only")
        _require(os.getpid() != 1 and os.getppid() == 1
                 and os.path.samefile('/proc/1/exe', '/usr/bin/bwrap'), "Worker requires its private bwrap PID owner")
    except OSError as error:
        raise RuntimeError("Worker containment facts are unavailable") from error


def dispatch(root, layout, *, keep_cache=False):
    """Supervise the entire internal worker, including all Python HTTP reads."""
    began = time.monotonic(); deadline = began + WHOLE_SECONDS
    _require(admit_layout(layout.profile, layout.receipt) == layout, "Dispatcher layout differs from mandatory admission")
    with ownership(layout):
        parent_mount = quota_facts(layout)
        layout.bootstrap.mkdir(mode=0o700)
        for name in ['home', 'tmp']:
            (layout.payload / name).mkdir(mode=0o700)
        environment = worker_environment(layout, deadline)
        worker_entry = ('import importlib.util,sys;'
            'spec=importlib.util.spec_from_file_location("xwin_setup_worker",sys.argv[1]);'
            'module=importlib.util.module_from_spec(spec);sys.modules[spec.name]=module;'
            'spec.loader.exec_module(module);sys.argv=sys.argv[1:];module.main(_hosted_worker=True)')
        args = ['/usr/bin/python3', '-I', '-B', '-c', worker_entry, str(root / 'scripts/setup-xwin.py'), '--hosted-runner',
                '--accept-license', '--cache-dir', str(layout.profile), '--receipt', str(layout.receipt)]
        if keep_cache: args.append('--keep-cache')
        output = run_checked(namespace(root, layout, worker=True, environment=environment) + args, cwd=layout.payload,
            environment=environment, timeout=deadline - time.monotonic(), runner=process_module(root).run)
        _require(quota_facts(layout) == parent_mount and time.monotonic() < deadline, "Parent quota/deadline changed")
        data = read_record(root, layout.receipt, 2 * 1024 ** 2)
        def unique(pairs):
            result = {}
            for key, value in pairs:
                _require(key not in result, "Duplicate worker receipt key")
                result[key] = value
            return result
        receipt = json.loads(data, object_pairs_hook=unique,
            parse_constant=lambda _: (_ for _ in ()).throw(RuntimeError('Nonfinite worker receipt')))
        receipt['hosted']['worker_mount'] = receipt['hosted']['mount']
        receipt['hosted']['mount'] = parent_mount
        receipt['hosted']['supervisor'] = {'whole_seconds': WHOLE_SECONDS, 'exit_code': 0, 'failure': None,
            'elapsed_seconds': time.monotonic() - began, 'output_bytes': len(output.encode()), 'pid_namespace': True,
            'max_output_bytes': MAX_OUTPUT, 'as_bytes': 1073741824, 'cpu_seconds': 600,
            'nproc': 128, 'nofile': 256, 'fsize': PAYLOAD_BYTES}
        write_json(layout.receipt, receipt, 2 * 1024 ** 2, replace=True)
        write_json(layout.locator, receipt, 2 * 1024 ** 2, replace=True)
        print(json.dumps({'location': str(layout.locator), 'receipt': str(layout.receipt), 'root': receipt['root']}))
