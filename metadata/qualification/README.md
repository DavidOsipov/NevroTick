# Portable metadata qualification

The full campaign remains incomplete. On 8 November the owner reopened the
bounded parser-budget repair and its affected checks; the seven-quote regression
and all 247 current metadata tests now pass. Fresh strict typing, causal controls
and subprocess-aware coverage also pass; every declared security module meets
its 90% line gate. This is not full mutation qualification. Read the [current amendment and historical restart checkpoint](CHECKPOINT.md)
and [W11-W10 task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
before another campaign. The old `/tmp` evidence directory is now absent;
its recorded partial outcomes are historical and cannot be re-admitted without
the original artifacts. The current repair has fresh evidence in the task's
existing coordinator chain.

This directory qualifies the Python source pipeline at MD-P1–MD-P3. The
[existing metadata plan](../../docs/plans/windows-package-metadata.md#13-metadata-pipeline-implementation--7-november-2026)
and [W11-W10 task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
own the requirements, candidate and actual results. Generated source, compiled
resources, native installation and release qualification are separate stages.

[manifest.json](manifest.json) declares all eight source identities, five
security modules and six critical files. Each security module needs at least
90% executable-line coverage. Each critical file needs at least 95% adjusted
viable mutation score. Branches are reported separately. Package markers have
no executable denominator; the field registry and Unicode table retain their
independent integrity and semantic controls. Tool generation limits and
uncovered functions/data must be reconciled explicitly.

Use CPython 3.12 on the qualified Linux/POSIX host. The three tool environments
are external to the checkout; qualification never installs into the system or
the runtime product. The lock files record exact distribution hashes. Reuse
admitted tools when their identities still match. An installed version string
alone is not tool admission.

1. Run strict typing against the admitted mypy 2.4.1 store. `typing.py` checks
   all eight production modules and a legal consumer, then requires five
   intended negative diagnostics. It reuses the selected installed-tool custody
   receipt and binds the current source/configuration separately:

   ```sh
   python3 -I -B metadata/qualification/typing.py \
     --checker-python /absolute/admitted-mypy-venv/bin/python \
     --tool-admission /absolute/prior-tool-admission.json \
     --output /absolute/fresh-typing-evidence
   ```

2. Measure the actual suite and its isolated CLI subprocesses with the admitted
   coverage.py 7.17.2 environment. Keep changed copied producer modules out of
   the healthy source denominator. Preserve raw per-process data, source/PID
   observations, test identities, branches and per-test reachability:

   ```sh
   python3 -I -B metadata/qualification/coverage.py \
     --python /absolute/admitted-coverage-venv/bin/python \
     --wheel /absolute/pinned-coverage-wheel.whl \
     --output /absolute/fresh-coverage-evidence
   ```

   `--controls-only` exercises child-process and changed-source admission; it
   does not qualify code coverage. The negative
   `--controls-only --missing-subprocess-control` invocation must fail.

3. Run Cosmic Ray 8.7.0 in disposable copies. Use the exact source-bound context
   map from step 2. The driver retains the complete default operator inventory,
   passing baseline, every generated variant, selected tests, raw logs and
   outcome. Unmeasured scope uses the complete suite. It never mutates the live
   checkout:

   ```sh
   /absolute/admitted-cosmic-ray-venv/bin/python -I -B \
     metadata/qualification/mutation.py \
     --python /absolute/admitted-cosmic-ray-venv/bin/python \
     --contexts /absolute/coverage-evidence/test-line-contexts.json \
     --context-binding /absolute/coverage-evidence/binding.json \
     --output /absolute/fresh-mutation-evidence \
     --workers 10 --timeout 180
   ```

   `--enumerate-only`, `--baseline-only`, `--limit` and `--ids` produce narrower
   evidence. None establishes the full per-file gate. Syntax/import failures,
   setup errors, skips and timeouts remain distinct from behavioral kills.
   Review every survivor; an equivalent exclusion needs a source-bound domain,
   output/error/effect argument and independent review. A favorable average or
   unexamined survivor cannot close a critical file.

Freeze sources, tests, inputs, tools, configuration and qualification helpers
before the final checkpoint. Preserve first failures and actual consumption.
After any repair, reconcile which prior results remain applicable and rerun
the affected evidence. Independently validate, generate and read back the
actual product source. No formal, Cargo, Windows, service, clock, signing or
Store operation is authorized by these commands.
