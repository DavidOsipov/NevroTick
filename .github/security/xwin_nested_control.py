"""Dedicated nested namespace control; selected hosted AppArmor profile required."""
from __future__ import annotations

import errno
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "scripts" / "tests"))
import test_xwin_hosted as shared

# bwrap v0.9.0 writes an ancestor limit, then checks another unshare refuses:
# https://github.com/containers/bubblewrap/blob/v0.9.0/bubblewrap.c#L3003-L3030
# The child's visible sysctl is recorded, not assumed to equal that ancestor's.


class NestedNamespaceControls(unittest.TestCase):
    def test_native_namespace_runs_inside_actual_worker(self) -> None:
        import time

        source_root = Path(__file__).resolve().parents[2]
        home = Path.home().resolve(strict=True)
        self.assertFalse(home.is_relative_to(Path("/tmp")) or home.is_relative_to(source_root),
                         "Unsupported nested fixture: HOME must be outside /tmp and the checkout")
        with tempfile.TemporaryDirectory(prefix="nevrotick-nested-xwin-", dir=home) as directory:
            profile = Path(directory)
            payload, bootstrap = profile / "payload", profile / "bootstrap"
            for path in [payload, bootstrap, payload / "tmp", payload / "home"]:
                path.mkdir(mode=0o700)
            layout = SimpleNamespace(profile=profile, payload=payload, bootstrap=bootstrap)
            inner = (
                "import ctypes,errno,json,os,pathlib,sys\n"
                "p,b=map(pathlib.Path,sys.argv[1:3])\n"
                "assert os.getuid()==os.geteuid()!=0\n"
                "with open('/proc/self/status','rb') as stream:raw=stream.read(16385)\n"
                "assert len(raw)<=16384\n"
                "state=dict(line.split(':',1) for line in raw.decode('ascii').splitlines() if ':' in line)\n"
                "cap_eff=int(state['CapEff'],16);no_new_privs=int(state['NoNewPrivs'])\n"
                "assert cap_eff==0 and no_new_privs==1,(cap_eff,no_new_privs)\n"
                "with open('/proc/self/attr/current','rb') as stream:label=stream.read(1025)\n"
                "assert 0<len(label)<=1024;label=label.decode('utf-8','replace').strip()\n"
                "before=tuple(os.stat('/proc/self/ns/'+name).st_ino for name in ['pid','user'])\n"
                "libc=ctypes.CDLL(None,use_errno=True);libc.unshare.argtypes=[ctypes.c_int];libc.unshare.restype=ctypes.c_int\n"
                "ctypes.set_errno(0);answer=libc.unshare(0x10000000);denial=ctypes.get_errno()\n"
                "assert answer==-1 and denial in (errno.EPERM,errno.ENOSPC),(answer,denial)\n"
                "assert tuple(os.stat('/proc/self/ns/'+name).st_ino for name in ['pid','user'])==before\n"
                "visible_max=None;max_error=None\n"
                "try:\n"
                " with open('/proc/sys/user/max_user_namespaces','rb') as stream:raw=stream.read(65)\n"
                " assert len(raw)<=64;visible_max=int(raw)\n"
                "except OSError as error:max_error=error.errno\n"
                "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ\n"
                "assert os.stat('/proc/self/ns/pid').st_ino!=int(sys.argv[3])\n"
                "assert os.stat('/proc/self/ns/user').st_ino!=int(sys.argv[4])\n"
                "assert pathlib.Path.cwd()==p and os.path.samefile('/tmp',p/'tmp')\n"
                "(p/'nested-payload-control').write_bytes(b'owned')\n"
                "pathlib.Path('/tmp/nested-tmp-control').write_bytes(b'tmp')\n"
                "assert (p/'tmp/nested-tmp-control').read_bytes()==b'tmp'\n"
                "assert (b/'worker-control').read_bytes()==b'public'\n"
                "for target in [pathlib.Path('/outside-owned-quota'),pathlib.Path('/dev/outside-owned-quota'),b/'native-forbidden']:\n"
                " try:target.write_bytes(b'forbidden')\n"
                " except OSError as error:assert error.errno in (errno.EROFS,errno.EACCES)\n"
                " else:raise AssertionError('nested write escaped payload')\n"
                "print(json.dumps({'status':'NESTED_NATIVE_SURFACE_OK','apparmor':label,'cap_eff':cap_eff,'no_new_privs':no_new_privs,'unshare_errno':denial,'visible_max_user_namespaces':visible_max,'max_read_errno':max_error},ensure_ascii=True))\n"
            )
            worker = (
                "import importlib.util,json,os,pathlib,stat,sys,types\n"
                "root,p,b=map(pathlib.Path,sys.argv[1:4])\n"
                "def load(filename,name):\n"
                " spec=importlib.util.spec_from_file_location(name,root/'scripts'/filename)\n"
                " m=importlib.util.module_from_spec(spec);sys.modules[name]=m;spec.loader.exec_module(m);return m\n"
                "h=load('xwin_hosted.py','nested_hosted');e=load('check-environment.py','nested_environment')\n"
                "with open('/proc/self/attr/current','rb') as stream:label=stream.read(1025)\n"
                "assert 0<len(label)<=1024;label=label.decode('utf-8','replace').strip()\n"
                "with open('/proc/self/status','rb') as stream:raw=stream.read(16385)\n"
                "assert len(raw)<=16384\n"
                "state=dict(line.split(':',1) for line in raw.decode('ascii').splitlines() if ':' in line)\n"
                "cap_eff=int(state['CapEff'],16);no_new_privs=int(state['NoNewPrivs'])\n"
                "print(json.dumps({'stage':'worker','apparmor':label,'cap_eff':cap_eff,'no_new_privs':no_new_privs},ensure_ascii=True),flush=True)\n"
                "assert cap_eff==0 and no_new_privs==1,(cap_eff,no_new_privs)\n"
                "h.require_worker_boundary()\n"
                "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ\n"
                "for path in [p.parent,p,b,p/'home',p/'tmp']:\n"
                " info=path.stat();assert info.st_uid==os.geteuid() and stat.S_IMODE(info.st_mode)==0o700\n"
                "assert {path.name for path in p.iterdir()}=={'home','tmp'}\n"
                "assert not any((p/'home').iterdir()) and not any((p/'tmp').iterdir())\n"
                "assert os.path.samefile('/tmp',p/'tmp')\n"
                "assert all(os.statvfs(root/'scripts'/name).f_flag&os.ST_RDONLY for name in ['xwin_hosted.py','check-environment.py'])\n"
                "(b/'worker-control').write_bytes(b'public')\n"
                "layout=types.SimpleNamespace(profile=p.parent,payload=p,bootstrap=b)\n"
                "env=h.worker_environment(layout,float(os.environ['NEVROTICK_XWIN_DEADLINE']))\n"
                "command=h.namespace(root,layout,worker=False,environment=env)+['/usr/bin/python3','-I','-B','-c',sys.argv[4],str(p),str(b),str(os.stat('/proc/self/ns/pid').st_ino),str(os.stat('/proc/self/ns/user').st_ino)]\n"
                "result=e.run(command,cwd=p,environment=env,timeout=5,max_output=16384)\n"
                "print(json.dumps({key:result[key] for key in ['exit_code','failure','output']},ensure_ascii=True),flush=True)\n"
                "assert result['failure'] is None and result['exit_code']==0\n"
                "assert json.loads(result['output'])['status']=='NESTED_NATIVE_SURFACE_OK'\n"
            )
            environment = shared.HOSTED.worker_environment(layout, time.monotonic() + 12)
            command = shared.HOSTED.namespace(source_root, layout, worker=True, environment=environment)
            result = shared.ENV.run([*command, "/usr/bin/python3", "-I", "-B", "-c", worker,
                              str(source_root), str(payload), str(bootstrap), inner], cwd=payload,
                             timeout=12, max_output=16384, environment=environment | {
                                 "NEVROTICK_SYNTHETIC_CREDENTIAL": "public-control-canary"})
            self.assertIsNone(result["failure"], result["output"])
            self.assertEqual(result["exit_code"], 0, result["output"])
            observed = [json.loads(line) for line in result["output"].splitlines()]
            self.assertEqual(len(observed), 2)
            self.assertEqual(observed[0]["stage"], "worker")
            self.assertTrue(observed[0]["apparmor"])
            self.assertEqual((observed[0]["cap_eff"], observed[0]["no_new_privs"]), (0, 1))
            self.assertEqual((observed[1]["exit_code"], observed[1]["failure"]), (0, None))
            native = json.loads(observed[1]["output"])
            self.assertEqual(native["status"], "NESTED_NATIVE_SURFACE_OK")
            self.assertTrue(native["apparmor"])
            self.assertEqual((native["cap_eff"], native["no_new_privs"]), (0, 1))
            self.assertIn(native["unshare_errno"], {errno.EPERM, errno.ENOSPC})
            self.assertEqual((payload / "nested-payload-control").read_bytes(), b"owned")
            self.assertEqual((payload / "tmp/nested-tmp-control").read_bytes(), b"tmp")
            print("XWIN_NESTED_OBSERVED " + json.dumps({"worker": observed[0], "native": native},
                                                     ensure_ascii=True), flush=True)


if __name__ == "__main__":
    unittest.main()
