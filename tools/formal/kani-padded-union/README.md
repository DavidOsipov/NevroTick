# Reviewed Kani padded-union repair

This source kit preserves the reviewed compiler patch, its ten binding regression
tests, and the exact eight-control Rust fixture for another Linux machine or a
future GitHub runner. It contains no binaries, SDK, vendor cache or installer.
[provenance.json](provenance.json) records immutable source and evidence identities.

The repair keeps a padded union member's logical payload type intact while
serializing a typed storage member followed by its offset-zero payload member.
It preserves lvalue/address/write behavior, substitution and single evaluation
of the base. Source-unit rebuilding controls do not establish complete TyCtxt
inliner coverage.

## Inputs

Use complete Kani 0.68.0 source at
[0d2328a93f0e0ff66132d6bfa1a7d884877cf862](https://github.com/model-checking/kani/tree/0d2328a93f0e0ff66132d6bfa1a7d884877cf862)
and [proposed-member-read-v2.patch](proposed-member-read-v2.patch), SHA256
d00920b70d101c66f3a83ba7f568276b7ed79b391e4a7bd86aba94e5e0fc83a6.
The manifest gives all three before/after source hashes and original Git blobs
checked against immutable GitHub metadata.

The qualified private build used nightly-2026-08-21-x86_64-unknown-linux-gnu,
Rust commit 8925ea358a0f265ca61026aadc7ecc506c545cbe, with matching rustc-dev,
rust-src and LLVM libraries. The upstream toolchain also requests llvm-tools
and rustfmt. A different SDK/nightly is a different candidate. The retained
Kani payload uses CBMC 6.11.0, source reference
[820ff0f555b43fb78e0cd9332e498461bd14244b](https://github.com/diffblue/cbmc/tree/820ff0f555b43fb78e0cd9332e498461bd14244b);
this patch rebuilds only kani-compiler.

Preserve the complete upstream Cargo.lock. The offline resolver closure used
291 registry packages, Charon b250680abd40ff1aaa07081d0497dc2755ed112e and
tracing-tree 841286bfffd3c2200810244506cd127013dbeff9. Charon was present for
workspace resolution while the selected compiler backend was cprover.
Do not trim optional workspace manifests to bypass source resolution.

## Private build recipe

These are the historical selected inputs/commands, not a replacement for
source/tool admission or bounded execution. Choose canonical, owned absolute
paths outside the repository and shared toolchains:

| Variable | Meaning |
| --- | --- |
| KANI_SOURCE_KIT | This directory |
| KANI_SOURCE | Complete private Kani source including pinned Charon |
| PRIVATE_TOOLCHAIN | Matching private nightly/SDK root |
| PRIVATE_RUSTUP_HOME | Parent of its toolchains directory |
| PRIVATE_CARGO_HOME | Fresh private Cargo home |
| REGISTRY_VENDOR | Authenticated 291-package directory source |
| GIT_VENDOR | Cargo directory source containing pinned tracing-tree |
| PRIVATE_TARGET | Owned build output |
| CC | Admitted absolute host linker; historical build used GCC 13 |

After checking base hashes, apply the exact patch only in that private source:

~~~sh
git -C "$KANI_SOURCE" apply --check "$KANI_SOURCE_KIT/proposed-member-read-v2.patch"
git -C "$KANI_SOURCE" apply "$KANI_SOURCE_KIT/proposed-member-read-v2.patch"
~~~

Verify all patched hashes. Preserve upstream .cargo/config.toml, including its
manifest-relative Kani paths, RUSTC_BOOTSTRAP=1 and lint settings. Do not inject
standalone proof RUSTFLAGS into this compiler build.

Instantiate [cargo-config.toml.in](cargo-config.toml.in) as the private
CARGO_HOME/config.toml. Replace its three tokens with JSON-quoted absolute path
strings, which are valid TOML basic strings here. Retain offline replacements,
target-applies-to-host=false, both explicit linker settings, and no other Cargo
ancestor/home configuration. For example, @CC_JSON@ becomes
"/usr/bin/x86_64-linux-gnu-gcc-13"; the other tokens name REGISTRY_VENDOR and
GIT_VENDOR.

Use a cleared, reviewed Cargo child environment with the following values.
Reject inherited wrappers, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, LD_PRELOAD,
LD_AUDIT and ambient loader/library overrides:

~~~text
PATH=<PRIVATE_TOOLCHAIN>/bin:/usr/bin:/bin
CARGO_HOME=<PRIVATE_CARGO_HOME>
CARGO_NET_OFFLINE=true
RUSTC=<PRIVATE_TOOLCHAIN>/bin/rustc
RUSTDOC=<PRIVATE_TOOLCHAIN>/bin/rustdoc
RUSTUP_HOME=<PRIVATE_RUSTUP_HOME>
RUSTUP_TOOLCHAIN=nightly-2026-08-21-x86_64-unknown-linux-gnu
LD_LIBRARY_PATH=<PRIVATE_TOOLCHAIN>/lib
LIBRARY_PATH=<PRIVATE_TOOLCHAIN>/lib
~~~

The two library settings belong only to that child and its descendants.
The unchanged upstream kani-compiler/build.rs emits both linker RPATHs:
PRIVATE_RUSTUP_HOME/toolchains/RUSTUP_TOOLCHAIN/lib and the literal
$ORIGIN/../toolchain/lib. Preserve those source-generated flags. The selected
host linker and its public OS libraries remain part of the trusted host.

From KANI_SOURCE, run these selected commands with that admitted environment:

~~~sh
"$PRIVATE_TOOLCHAIN/bin/cargo" -Zhost-config -Ztarget-applies-to-host \
  test --manifest-path "$KANI_SOURCE/Cargo.toml" -p cprover_bindings --lib \
  --release --locked --offline --target x86_64-unknown-linux-gnu \
  --target-dir "$PRIVATE_TARGET" -j2 -- --test-threads=1

"$PRIVATE_TOOLCHAIN/bin/cargo" -Zhost-config -Ztarget-applies-to-host \
  build --manifest-path "$KANI_SOURCE/Cargo.toml" -p kani-compiler \
  --bin kani-compiler --no-default-features --features kani-compiler/cprover \
  --release --locked --offline --target x86_64-unknown-linux-gnu \
  --target-dir "$PRIVATE_TARGET" -j2
~~~

Historically the first target was fresh; the second reused only its authenticated
snapshot and required the compiler output to be absent. Tests used 600-second
wall/CPU bounds; the build used 1200 seconds. Both used 8 GiB address space per
process, 128 tasks and two build jobs. The 8 GiB free-space floor and 3 GiB target
monitor were soft checks, not aggregate quotas. These command snippets do not
implement containment, output limits, admission, accounting or cache custody.

## Matching LLVM layout

Cargo can load the rustc_driver copy in the host target-library directory.
Before sealing a new private candidate, copy the already authenticated regular
library byte-for-byte, without replacing an existing file or using a symlink:

~~~text
from: <PRIVATE_TOOLCHAIN>/lib/libLLVM.so.23.1-rust-1.100.0-nightly
to:   <PRIVATE_TOOLCHAIN>/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM.so.23.1-rust-1.100.0-nightly
~~~

Both must be exactly 207,996,272 bytes, SHA256
559bc1644a9b1124bf62bf11cb022c8c385cb53dc940893a8a87b44241f9a77d.
Reject a conflicting destination and preserve existing linker scripts.
Reinventory and admit the entire resulting closure. This historical correction
added one regular file, increasing 7,130 entries to 7,131; prior receipts remain
historical.

## Controls and evidence limits

The private build passed all 46 binding tests, including the ten new tests, with
no failed, ignored or filtered tests. Its eight Kani controls used the verbatim
[fixture](member-controls/controls.rs) and
[fixed specification](member-controls/schedule.json), in this order:

| Job | Harness | Required outcome |
| --- | --- | --- |
| 1 | padded_array_read | Healthy assertions and both covers |
| 2 | padded_scalar_read | Healthy assertions and both covers |
| 3 | initialized_union_write_read | Healthy assertions and both covers |
| 4 | raw_address_write_read | Healthy assertions and both covers |
| 5 | nested_payload_field | Healthy assertions and both covers |
| 6 | equal_size_neighbor | Healthy assertions and both covers |
| 7 | false_retention_control | Sole failure: deliberately false padded retention; positive predicate succeeds |
| 8 | padded_scalar_read | Byte-identical restored healthy control |

Use the fixed schedule's full domains, named assertions/covers and unchanged
memory/overflow/pointer/undefined-function/unwinding/sanity checks. Its standalone
profile is unwind1, CaDiCaL, 60-second solver/180-second outer bounds, 8 GiB address
space and 128 tasks. The copied schedule is an original preparation specification;
its source-only status is not the qualification receipt. Historical qualification
receipt identities are in provenance.json; binaries/receipts remain outside Git.

The public EraWindow/Garde constructor proof remains unfinished. No
restrict-vtable precision profile is adopted here. A fresh compiler needs
complete candidate admission and fresh controls. This kit's ordinary Git
reverse/forward checks establish source portability, not a new native build,
general verifier adoption, product proof, automatic GitHub CI or release readiness.

The Kani-derived patch retains Apache-2.0 OR MIT terms. Verbatim
[Apache licence](LICENSE-APACHE) and [MIT licence](LICENSE-MIT) copies are included;
preserve upstream headers/notices. Continue through the
[formal workflow](../README.md) and [S1.1 task](../../../tasks/s1.1-time-domain.md).
