# NevroTick Slint Core calendar patch

This directory derives from the published `i-slint-core 1.18.1` archive,
SHA-256 `ed2ea15059d70bc31b1a8b1eb4c34f2c5df04cb7e53f36179b43c0261231d646`.
It selects the upstream GPL-3.0-only grant. The original normalized
manifest is retained in `Cargo.toml.upstream`. Cargo metadata, source
and upstream licence files beyond the manifest and calendar module retain
their published bytes. The patch replaces the unmaintained Chrono crate
with `time 0.3.55` under the GNU and MSVC desktop UI graph.

The original supported year interval, weekday numbering, date parts,
calendar failure values and default date-picker patterns are retained.
The adapter bounds text and format input to 1,024 bytes. The default
`%m/%d/%Y` parser retains Chrono's numeric widths, leading whitespace
and signed-year behavior within that bound. Other input formats must
describe exactly one Gregorian year, month and day with literals.
This avoids accepting a date while silently ignoring a contradictory
weekday or a second date component. Formatting follows time's strftime
grammar. Chrono-specific directives and alternate date representations
are outside this derivative's supported pattern set. Date formatting
requires std. This derivative expressly selects the desktop std build.
Local date reads the platform's active offset. An unavailable offset
returns the existing unavailable-date sentinel.
