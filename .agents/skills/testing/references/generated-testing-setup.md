# Prepare a missing generated-testing framework

Use when the [selection guide](generated-testing-selection.md) identifies a
concrete input, history or schedule gap. Turn a missing package,
source cache, interpreter or runner into scoped preparation work. Read the
[testing contract](../CONTRACT.md) and [configured tooling](configured-tooling.md).

Setup guidance reviewed **16 November 2026, Asia/Tbilisi**. Versions are
researched candidates, not installed or admitted NevroTick selections. The QA
skills themselves live in this repository; installing their referenced libraries
is separate work on the selected execution host.

## Contents

- [Classify the missing prerequisite](#classify-the-missing-prerequisite)
- [Rust libraries and the actual test build](#rust-libraries-and-the-actual-test-build)
- [Hypothesis in an owned Python environment](#hypothesis-in-an-owned-python-environment)
- [Verify readiness and return to the real defect](#verify-readiness-and-return-to-the-real-defect)
- [Sources](#sources)

## Classify the missing prerequisite

Use current task authorization and earlier owner decisions. A skill neither
grants new authority nor creates another permission round. Complete already
authorized necessary dependency or environment preparation. If a specific
unresolved decision blocks adoption, finish the concrete package/graph,
target, runner and budget proposal first; identify that remaining decision and
its authoritative source. Keep explicit holds scoped to the affected execution.

| Observed state | Action |
| --- | --- |
| Selected graph/environment is already admitted; local sources or packages are absent | Acquire/reconstruct that exact approved selection through its existing owner. Do not upgrade merely because setup is needed. |
| Files or package metadata exist, but identity/features/runner admission is incomplete | Establish the missing binding or readiness step; cache presence alone proves no usable test. |
| Framework is new to this product or environment | Prepare the dependency graph, integration seam and runner change under the existing task. Rust and Python routes below have different owners. |
| Compiler/interpreter itself is missing | Select the existing environment bootstrap and its exact pins. [WSL setup](../../../../TOOLING.md#repeatable-wsl-setup-and-checks) and [Windows setup](../../../../TOOLING.md#windows-development-route) own Rust prerequisites; inspect actual Python provisioning separately. |
| Framework adds no useful exploration beyond existing tests | Use the existing deterministic driver; dependency adoption is unnecessary for that question. |
| A relevant run is explicitly held | Continue permitted source/harness preparation and separately permitted tests. Keep every affected held result incomplete; continue unrelated authorized work. |

Do not treat package absence as a permanent project block. `setup-unix.py` installs
its fixed tool manifest, not arbitrary frameworks; do not invoke the whole tool
installer just because a test library is missing. Inspect the actual selected
host/environment, not another checkout's cached installation claim.

## Rust libraries and the actual test build

Shuttle, Proptest and proptest-state-machine are Cargo libraries. Their setup
changes the dependency graph and test build; `cargo install` manages
executable targets and is not their installation route. Distinguish
`awslabs/shuttle` from the unrelated Shuttle cloud platform.

### Discover and choose the manifest

Read source and configuration without executing package code:

```sh
rg -n 'shuttle|proptest' Cargo.toml Cargo.lock crates/*/Cargo.toml
rg -n 'shuttle|proptest|cfg.*test' crates --glob '*.rs'
```

Inspect aliases, workspace inheritance, the actual owning crate and selected
target/features; an unrelated vendored lock does not establish product selection.
Inspect toolchain and dependency policy too. `cargo test -- --list` compiles and
executes a test binary, so it is not this initial read-only discovery step.
Frozen Cargo metadata/feature-tree inspection belongs to an available admitted
graph; it must not silently fetch a missing source.

For an ordinary Rust property, the researched candidate is:

```toml
[dev-dependencies]
proptest = "=1.12.0"
```

Add `proptest-state-machine = "=0.9.0"` to that section only for selected generated
histories. Both reviewed manifests declare Rust 1.88. Proptest defaults include
`std`, `fork`, `timeout` and `bit-set`; the state-machine dependency enables those
features too. A separately reviewed ordinary-property-only declaration can use
`proptest = { version = "=1.12.0", default-features = false, features = ["std"] }`,
which supplies no Proptest fork/timeout facility. Feature unification still
applies; disabling defaults on one edge cannot disable another enabling edge.
The optional `attr-macro` is unnecessary for the examples in the
[Proptest manual](proptest-state-machines.md).

A test-only Shuttle driver can use `shuttle = "=0.9.10"` in `[dev-dependencies]`.
For an integration test whose production library selects instrumented imports,
the dependency must instead be available to that normal library build:

```toml
[dependencies]
shuttle = { version = "=0.9.10", default-features = false, optional = true }

[features]
shuttle = ["dep:shuttle"]
```

Candidate snippets, not existing manifest changes. Connect the feature
to the actual import seam in the [Shuttle manual](shuttle-concurrency.md).
The library consumed by an integration test normally builds without `cfg(test)`;
`all(test, feature = "shuttle")` can leave it uninstrumented. Shuttle's reviewed
defaults are empty and its manifest declares no `rust-version`; inspect actual
transitive compiler/platform requirements. Preserve the selected shipping Rust.

For Tokio, separately review the `shuttle-tokio` dependency alias and
`tokio/shuttle` forwarding. The wrapper's `tokio = "1"` edge does not by itself
preserve NevroTick's exact original Tokio pin. Compare normal and instrumented
graphs, the full wrapper/inner cohort and actual modeled operations; do not
replace all runtime dependencies merely to copy a tutorial.

### Resolve, acquire and admit the runner

Follow [dependency updates](../../environment/dependency-updates/SKILL.md) and
[TOOLING's guarded graph procedure](../../../../TOOLING.md#registry-dependency-cooldown--selected-13-november-2026).
For an authorized manifest addition, use an owned candidate containing real
source/manifests, accepted lock/config and proposed changes. Variables below
represent task-owned paths, not existing environment variables. The quota
filesystem must satisfy the existing private empty 512 MiB/16,384-inode limits.

```sh
python3 -B scripts/dependency_cooldown.py \
  --candidate-root "$NV_CANDIDATE" --baseline-lock "$NV_ACCEPTED_LOCK" \
  --operation workspace --toolchain-dir "$NV_TOOLCHAIN" \
  --quota-root "$NV_QUOTA" --output-dir "$NV_NEW_RESOLUTION"
```

This deliberately resolves metadata. Review the complete new
direct/transitive/host/build graph, registry publication evidence under 14-day/deny,
source, checksums, licences, features and advisories. A generated lock/decision
does not approve itself. Use the existing concrete exception procedure only when
applicable; exact direct pins do not freeze transitives or establish their age.

Reconcile accepted manifests/lock, dependency policy and its code-owned checkpoint.
The [current verifier](../../../../scripts/verify.py) also checks exact member
dependencies/features: prepare the matching runner row through the existing
owner. A new TOML entry alone does not make that row executable.

Next, follow TOOLING's exact acquisition/dependency commands: `prepare-product.py`
for approved archives, fresh advisory acquisition, `verify.py --lane dependencies`,
then the selected frozen implementation runner. Use new external destinations;
`--configure` is for first setup of the absent private pointer. `--frozen` prevents
acquisition; `--locked` alone does not mean offline. Do not invent
`verify.py --features shuttle` or `check-analysis.py --tool proptest`.
The selected manual then owns nonempty discovery, controls and actual test/replay.

## Hypothesis in an owned Python environment

The inspected checkout has no general Hypothesis environment or lock. Existing
typing/coverage/mutation environments and locks belong to their named tasks;
do not extend the fixed mypy environment in `policy/python-typing.json`.
Choose an owned environment and runner in the existing Python test task.
This recipe proposes reproducible setup; it creates no new global Python policy
and does not apply Cargo's registry cooldown to PyPI.

The [reviewed Hypothesis 6.170.15 manifest](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/pyproject.toml)
requires Python 3.11+ and `sortedcontainers>=2.1.0,<3.0.0`. It includes a native
extension and uses Maturin for source builds. Select compatible wheels for the
actual Python implementation/version/ABI and platform; do not silently fall back
to an sdist, native compiler install or another Python version.

### Detect the missing prerequisite

| Observation | Next action |
| --- | --- |
| Interpreter absent or incompatible | Prepare the selected interpreter through its environment owner. |
| `venv`/`ensurepip` unavailable or bootstrap fails | Preserve the actual diagnostic and use the distributor's approved repair. Missing base-interpreter pip alone need not block venv's own bootstrap. |
| Selected environment's `-m pip --version` fails | Repair that environment's pip before package acquisition; do not redirect installation to a different Python. |
| `importlib.metadata.version("hypothesis")` raises `PackageNotFoundError` | Hypothesis distribution metadata is absent from that interpreter's environment. |
| Metadata exists, but import or bounded control fails | Installed but unusable/incomplete; diagnose the actual failure, including native compatibility. |

With the trusted selected interpreter, use
`-I -B -c "from importlib.metadata import version; print(version('hypothesis'))"`.
Other exceptions do not establish absence. Repeat for pytest only when selected.
A missing `hypothesis` shell command is not the library check; the CLI needs
optional dependencies. Choose direct invocation/unittest or pytest deliberately.
The [worked admission example](hypothesis-admission.md#worked-example-physical-files-and-a-real-reader)
uses pytest; its Hypothesis plugin ships with Hypothesis, so do not guess and
install another plugin distribution or the broad `all` extra.

### Create an isolated environment

Use a reviewed base interpreter and a new owned external destination. Run steps
separately and stop on failure; confirm the destination is absent first. All paths
below are placeholders. A venv separates packages; the admitted bounded runner
supplies process/network/resource isolation. Activation is unnecessary, including
on PowerShell; no execution-policy change or automatic pip upgrade is required.

Choose host from the actual consumer's import and I/O requirements. The
[worked Engine-reader example](hypothesis-admission.md#worked-example-physical-files-and-a-real-reader)
requires Linux/WSL and its Linux interpreter because its tool closure uses
Unix-only APIs. The native Windows recipe below prepares Hypothesis for a
separately selected Windows-compatible consumer, not that unchanged reader.

```bash
NV_BASE_PYTHON='/absolute/reviewed/python3'
NV_ENV='/absolute/new/private/hypothesis-env'
"$NV_BASE_PYTHON" -I -B --version
"$NV_BASE_PYTHON" -I -B -m venv "$NV_ENV"
NV_PY="$NV_ENV/bin/python"
"$NV_PY" -I -B -m pip --version
```

```powershell
$NvBasePython = 'C:\reviewed\Python\python.exe'
$NvEnv = 'C:\new-private\hypothesis-env'
& $NvBasePython -I -B --version
& $NvBasePython -I -B -m venv $NvEnv
$NvPy = Join-Path $NvEnv 'Scripts\python.exe'
& $NvPy -I -B -m pip --version
```

### Select and freeze the package closure

Record the installer version too; use options supported by that selected pip.
If no reviewed requirements exist, prepare a bounded wheel-only resolution from
the intended HTTPS index using the target interpreter. Inspect effective pip
configuration without exposing credentials. `--isolated` ignores environment/user
configuration, but is not a guarantee that global/environment-local configuration
has disappeared. Exclude unreviewed extra indexes, remote requirements, local
projects, editable sources and source distributions from this recipe.

For the pytest recipe, set `NV_INDEX`, exact `NV_PYTEST_VERSION` and a new external
`NV_PROPOSAL` path first. Omit the pytest argument for a direct-only environment:

```bash
"$NV_PY" -I -B -m pip --isolated --disable-pip-version-check --no-cache-dir --no-input install --dry-run --ignore-installed --only-binary=:all: --retries 0 --timeout 15 --index-url "$NV_INDEX" --report "$NV_PROPOSAL" "hypothesis==6.170.15" "pytest==$NV_PYTEST_VERSION"
```

PowerShell uses the same arguments after `& $NvPy`; supply its corresponding
`$NvIndex`, `$NvProposal` and `$NvPytestVersion` values. This operation can contact
the index and download artifacts; separately authorized bounded acquisition owns
time, bytes, files and output. A socket timeout is not a total deadline.

Review every resolved distribution, target-compatible wheel, source and SHA-256.
The [pip report](https://pip.pypa.io/en/stable/reference/installation-report/)
is proposal evidence, not an installation lock or acceptance decision. Produce
a reviewed requirements file containing the complete exact dependency closure,
with each row `name==version --hash=sha256:REVIEWED_DIGEST`. Resolve missing hashes;
never invent them. Upstream ranges and `pip freeze` alone do not supply that
closure. Keep separate target-specific records where markers/wheels differ.

### Acquire approved wheels and install offline

Set `NV_REQUIREMENTS`/`$NvRequirements` to that actual reviewed file and
`NV_WHEELS`/`$NvWheels` to a new owned wheel directory. These artifacts are setup
inputs, not files already provided by this documentation. Use the same target
interpreter; another target needs its explicitly reviewed pip selectors.

```bash
"$NV_PY" -I -B -m pip --isolated --disable-pip-version-check --no-cache-dir --no-input download --require-hashes --only-binary=:all: --retries 0 --timeout 15 --index-url "$NV_INDEX" --dest "$NV_WHEELS" -r "$NV_REQUIREMENTS"
```

```powershell
& $NvPy -I -B -m pip --isolated --disable-pip-version-check --no-cache-dir --no-input download --require-hashes --only-binary=:all: --retries 0 --timeout 15 --index-url $NvIndex --dest $NvWheels -r $NvRequirements
```

Reconcile the complete acquired wheel set with the approved graph/hashes before
installation. Install with network access denied and local name/version/hash rows
only; `--no-index` alone does not prohibit direct URLs or enforce network isolation.

```bash
"$NV_PY" -I -B -m pip --isolated --disable-pip-version-check --no-cache-dir --no-input install --require-hashes --only-binary=:all: --no-index --find-links "$NV_WHEELS" -r "$NV_REQUIREMENTS"
"$NV_PY" -I -B -m pip --isolated --disable-pip-version-check check
```

```powershell
& $NvPy -I -B -m pip --isolated --disable-pip-version-check --no-cache-dir --no-input install --require-hashes --only-binary=:all: --no-index --find-links $NvWheels -r $NvRequirements
& $NvPy -I -B -m pip --isolated --disable-pip-version-check check
```

Compare actual interpreter, distribution versions and installed graph with the
reviewed inputs. `pip check` establishes dependency compatibility, not file
integrity or successful Hypothesis/native execution. Retain the installation
inventory and whatever admission bindings the selected runner requires.

## Verify readiness and return to the real defect

Report these stages separately: declared, resolved, acquired/installed, admitted
runner, compiled/imported, discovered, executed controls, product scenario, replay.
Complete the needed stages; do not repeatedly ask for authorization already
provided by the task. An explicit remaining hold or concrete prerequisite stays
visible without blocking unrelated useful work.

For Rust, use the selected manual's separate healthy/fault controls and actual
production scenario through its admitted runner. For Python, first use the exact
environment interpreter for a bounded import/generated-property control, then
the selected real-reader test on its supported host. The Engine-reader example
uses the Linux/WSL environment identified above. Require a healthy result, positive invocation
count, intended fault assertion and propagated failing exit. Import failure,
missing native extension, setup failure or timeout cannot satisfy the fault
control. Use fresh state and an external aggregate deadline.

When using pytest, verify its actual configuration/plugin loading and exact nodes
through the [Hypothesis execution procedure](hypothesis-admission.md#select-execute-and-observe).
`--trace-config` exposes active plugins; if autoload is intentionally disabled,
explicitly load the bundled `_hypothesis_pytestplugin` once using `-p`.
Direct invocation of a decorated function does not qualify pytest integration.
Collection executes imports but does not run the property; zero selection is a gap.

Retain concrete failure input/history/schedule and successful clean replay, then
return to the original defect question. Setup controls establish tool mechanics;
the independent contract oracle and actual production boundary establish the
useful semantic evidence. Do not replace a held or absent product run with an
installation success claim.

## Sources

- [Cargo installation](https://doc.rust-lang.org/cargo/commands/cargo-install.html), [development dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies), [integration tests](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).
- [Pinned Shuttle manifest](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle/Cargo.toml), [Tokio wrapper manifest](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/wrappers/shuttle-tokio/Cargo.toml).
- [Pinned Proptest manifest](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/Cargo.toml), [state-machine manifest](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/Cargo.toml).
- [Python venv](https://docs.python.org/3/library/venv.html), [ensurepip](https://docs.python.org/3/library/ensurepip.html), [Hypothesis quickstart](https://hypothesis.readthedocs.io/en/latest/quickstart.html).
- [pip secure installs](https://pip.pypa.io/en/stable/topics/secure-installs/), [install](https://pip.pypa.io/en/stable/cli/pip_install/), [download](https://pip.pypa.io/en/stable/cli/pip_download/), [check](https://pip.pypa.io/en/stable/cli/pip_check/), [configuration](https://pip.pypa.io/en/stable/topics/configuration/).
- [Hypothesis pytest integration](https://hypothesis.readthedocs.io/en/latest/reference/integrations.html#the-hypothesis-pytest-plugin), [pytest plugin controls](https://docs.pytest.org/en/stable/how-to/plugins.html).
