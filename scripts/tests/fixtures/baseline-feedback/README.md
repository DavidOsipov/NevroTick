# Historical baseline-feedback parser inputs

These 13 immutable source/manifest fixtures preserve the exact preimages used by
the existing baseline-feedback compiler, mutation-diff and review recognizers.
They are parser-test data, not fresh compiler results, proof evidence or current
product qualification. Current-source tests keep their current source inputs.

The twelve Rust source files were captured verbatim from repository commit
`8251880a7223e5aa591a317c51196c13316fb7f6`. The Rust 1.99.1 tool manifest was
captured verbatim from `8fce18b30956c539749408f6cb21c13729ba92fb`; its SHA-256 is
`83deaf865a9b5df42b02a5b9b31ec0f1af3fdd3cbdd291fb064256b88a4d1d41`.
All source hashes match the unchanged legacy role/review constants.

The `.txt` suffix identifies inert fixtures. Original source paths are retained
below this directory because line positions, byte offsets and complete unified
diff preimages are part of those historical contracts. Source copyright and
licence notices remain verbatim. Combined fixture payload: 531,604 bytes.

`historical_source_bytes()` in `test_baseline_feedback_analysis.py` uses the
existing bounded descriptor-based reader, rejects linked/replaced/nonregular
files and verifies each fixture against its historical role/review hash before
use. CI uses these checked-in bytes without Git/history lookup, network, private
reports, compilation or archived build artifacts. Updating the product source
does not update or qualify these historical recognizers.
