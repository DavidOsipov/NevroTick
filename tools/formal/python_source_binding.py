"""Bind repository Python execution to source before proof-input freezing.

This is a source/code correspondence check within the trusted Python interpreter,
not protection against a compromised process. Direct loads compile their exact
bounded source buffer; the scoped observer also checks older transitive loaders
before their module bodies run, including loaders that read existing bytecode.
"""
from __future__ import annotations

import sys

_MODULE_CODE = sys._getframe().f_code

from contextlib import contextmanager
import hashlib
import importlib.util
import os
from pathlib import Path
import stat


MAX_SOURCE_BYTES = 2 * 1024**2
_MISSING = object()


class SourceBindingError(RuntimeError):
    """The executing source closure or its subsequent custody is unestablished."""


class SourceBinding:
    """Track a finite repository module closure without changing global loaders.

    Construction binds this helper and the supplied executing module frame.
    Import observation is synchronous on the calling thread, matching these
    controllers' module initialization. Loaded module code must not create
    independent import threads; the declared source inventory is checked before
    freezing or dispatch. Standard-library code remains in the interpreter TCB.
    """

    def __init__(self, root: Path, frame):
        root = Path(root)
        if (not root.is_absolute() or root != root.resolve(strict=True)
                or not root.is_dir()
                or any(item.is_symlink() for item in (root, *root.parents))):
            raise SourceBindingError("Source root redirects or is not a directory")
        self.root = root
        self._sources = {}
        self._depth = 0
        self._previous_profile = None
        self._bind_code(Path(__file__), _MODULE_CODE)
        self.bind_frame(frame)

    def _path(self, value):
        path = Path(value)
        if (not path.is_absolute() or not path.is_relative_to(self.root)
                or path == self.root or path != path.resolve(strict=True)
                or any(item.is_symlink() for item in (path, *path.parents))):
            raise SourceBindingError("Python source path escapes or redirects")
        return path

    def _read(self, value):
        """Read one owned, unaliased regular inode without following redirects."""
        path = self._path(value)
        before_path = path.lstat()
        descriptor = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            before = os.fstat(descriptor)
            if (not stat.S_ISREG(before.st_mode) or before.st_uid != os.geteuid()
                    or before.st_nlink != 1 or before.st_size > MAX_SOURCE_BYTES
                    or (before.st_dev, before.st_ino) != (before_path.st_dev, before_path.st_ino)):
                raise SourceBindingError("Python source is special, aliased, unowned or oversized")
            with os.fdopen(descriptor, "rb", closefd=False) as stream:
                data = stream.read(MAX_SOURCE_BYTES + 1)
            after = os.fstat(descriptor)
            after_path = path.lstat()
            fields = ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
                      "st_size", "st_mtime_ns", "st_ctime_ns")
            identity = lambda item: tuple(getattr(item, name) for name in fields)
            if (len(data) > MAX_SOURCE_BYTES or len(data) != before.st_size
                    or identity(before) != identity(after)
                    or identity(before) != identity(after_path)):
                raise SourceBindingError("Python source changed while reading")
            return path, data
        finally:
            os.close(descriptor)

    def _record(self, path, data):
        name = path.relative_to(self.root).as_posix()
        current = {"sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}
        previous = self._sources.get(name)
        if previous is not None and previous != current:
            raise SourceBindingError("Previously loaded Python source changed: " + name)
        self._sources[name] = current

    def _bind_code(self, value, code):
        path, data = self._read(value)
        expected = compile(data, str(path), "exec", dont_inherit=True, optimize=sys.flags.optimize)
        if code.co_filename != str(path) or code != expected:
            raise SourceBindingError("Executing Python code differs from source: " + str(path))
        self._record(path, data)

    def bind_frame(self, frame):
        """Bind an executing controller/wrapper module, including its nested code."""
        if frame.f_code.co_name != "<module>":
            raise SourceBindingError("An executing module frame is required")
        self._bind_code(Path(frame.f_code.co_filename), frame.f_code)

    def _profile(self, frame, event, argument):
        if event == "call" and frame.f_code.co_name == "<module>":
            code_path = Path(frame.f_code.co_filename)
            origin = frame.f_globals.get("__file__")
            origin_path = Path(origin) if isinstance(origin, str) else code_path
            if origin_path.is_absolute() and origin_path.is_relative_to(self.root):
                self._bind_code(origin_path, frame.f_code)
            elif code_path.is_absolute() and code_path.is_relative_to(self.root):
                self._bind_code(code_path, frame.f_code)
        previous = self._previous_profile
        if previous is not None:
            previous(frame, event, argument)

    @contextmanager
    def _observe(self):
        """Scope transitive observation and restore the exact prior profile hook."""
        outer = self._depth == 0
        if outer:
            self._previous_profile = sys.getprofile()
            sys.setprofile(self._profile)
        self._depth += 1
        try:
            yield
            if sys.getprofile() != self._profile:
                raise SourceBindingError("Python import observation was replaced")
        finally:
            self._depth -= 1
            if outer:
                previous = self._previous_profile
                self._previous_profile = None
                sys.setprofile(previous)

    def load_module(self, name, path):
        """Execute the exact source buffer, bypassing bytecode and cached modules."""
        if not isinstance(name, str) or not name:
            raise SourceBindingError("A nonempty module name is required")
        path, data = self._read(path)
        code = compile(data, str(path), "exec", dont_inherit=True, optimize=sys.flags.optimize)
        self._record(path, data)
        spec = importlib.util.spec_from_file_location(name, path)
        if spec is None or spec.loader is None:
            raise SourceBindingError("Python module specification is unavailable")
        module = importlib.util.module_from_spec(spec)
        module.__source_binding__ = self
        previous = sys.modules.get(name, _MISSING)
        sys.modules[name] = module
        try:
            with self._observe():
                exec(code, module.__dict__)
        except BaseException:
            if previous is _MISSING:
                sys.modules.pop(name, None)
            else:
                sys.modules[name] = previous
            raise
        return module

    def verify(self, names, manifest=None):
        """Reject undeclared or changed executed sources before freezing/dispatch."""
        declared = set(names)
        if any(not isinstance(name, str) or Path(name).is_absolute()
               or ".." in Path(name).parts for name in declared):
            raise SourceBindingError("Declared source inventory is not repository-relative")
        missing = self._sources.keys() - declared
        if missing:
            raise SourceBindingError("Executed Python source missing from inventory: " + ", ".join(sorted(missing)))
        for name, expected in self._sources.items():
            _, data = self._read(self.root / name)
            if {"sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)} != expected:
                raise SourceBindingError("Loaded Python source changed: " + name)
            if manifest is not None:
                entry = manifest.get(name) if isinstance(manifest, dict) else None
                if not isinstance(entry, dict) or entry.get("sha256") != expected["sha256"]:
                    raise SourceBindingError("Frozen Python source differs from execution: " + name)

    def snapshot(self):
        """Return independent records; callers cannot rewrite captured identities."""
        return {name: dict(value) for name, value in self._sources.items()}
