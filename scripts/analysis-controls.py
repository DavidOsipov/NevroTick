"""Independent tool controls, generated only in an owned external workspace.

These are analyzer fixtures, never application code, dependency admission or a
product test matrix. Seeded defects must fail for their intended diagnostics.
"""

from pathlib import Path


MIRI = '''
#[test]
fn healthy_owned_value() {
    let value = Box::new(41_u32);
    assert_eq!(*value, 41);
}

#[test]
fn seeded_use_after_free() {
    let pointer = Box::into_raw(Box::new(41_u32));
    unsafe {
        drop(Box::from_raw(pointer));
        assert_eq!(*pointer, 41);
    }
}
'''

LOOM = '''
#[cfg(test)]
mod tests {
    use loom::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    use loom::thread;

    fn explore(faulty: bool) {
        let mut builder = loom::model::Builder::new();
        builder.max_threads = 3;
        builder.max_branches = 100;
        builder.preemption_bound = Some(2);
        builder.check(move || {
            let count = Arc::new(AtomicUsize::new(0));
            let workers: Vec<_> = (0..2).map(|_| {
                let count = count.clone();
                thread::spawn(move || {
                    if faulty {
                        let old = count.load(Ordering::SeqCst);
                        count.store(old + 1, Ordering::SeqCst);
                    } else {
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                })
            }).collect();
            for worker in workers { worker.join().unwrap(); }
            assert_eq!(count.load(Ordering::SeqCst), 2, "both increments must survive");
        });
    }

    #[test]
    fn healthy_atomic_update() { explore(false); }

    #[test]
    fn seeded_lost_update() { explore(true); }
}
'''

FUZZ = '''#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() != 1 { return; }
    let byte = data[0];
    let observed = DECODE;
    // Independent wider arithmetic states the signed-byte boundary contract.
    let expected = if byte < 128 { i16::from(byte) } else { i16::from(byte) - 256 };
    assert_eq!(observed, expected, "signed byte decoding must preserve negative offsets");
});
'''


def create(destination: Path) -> dict[str, Path]:
    """Create fresh isolated Cargo fixtures without touching an application."""
    if not destination.is_absolute() or destination.exists():
        raise ValueError("Use a fresh absolute fixture destination.")
    destination.mkdir(mode=0o700)
    projects = {}
    for name in ("miri", "loom", "fuzz", "vet-healthy", "vet-unreviewed"):
        project = destination / name
        (project / "src").mkdir(mode=0o700, parents=True)
        manifest = (f'[package]\nname = "nevrotick-{name}-control"\nversion = "0.0.0"\n'
                    'edition = "2024"\npublish = false\n\n[workspace]\n')
        if name in ("loom", "vet-unreviewed"):
            manifest += '\n[dependencies]\nloom = { version = "=0.7.2", default-features = false }\n'
        if name == "fuzz":
            manifest += ('\n[package.metadata]\ncargo-fuzz = true\n'
                         '\n[dependencies]\nlibfuzzer-sys = "=0.4.13"\n')
            for case in ("healthy", "defect"):
                manifest += (f'\n[[bin]]\nname = "{case}"\npath = "fuzz_targets/{case}.rs"\n'
                             'test = false\ndoc = false\nbench = false\n')
            (project / "fuzz_targets").mkdir()
            for case, expression in (("healthy", "i16::from(i8::from_ne_bytes([byte]))"),
                                     ("defect", "i16::from(byte)")):
                (project / f"fuzz_targets/{case}.rs").write_text(FUZZ.replace("DECODE", expression))
        (project / "Cargo.toml").write_text(manifest)
        (project / "src/lib.rs").write_text(MIRI if name == "miri" else LOOM if name == "loom" else "")
        projects[name] = project
    return projects
