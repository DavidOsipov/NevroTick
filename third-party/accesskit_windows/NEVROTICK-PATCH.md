# NevroTick source-only AccessKit Windows candidate

This is the source-only NevroTick derivative of accesskit_windows 0.34.0 from
AccessKit revision c88605b96d04431f9c3c792464a0f2f253480e94. Its original crate
SHA-256 is 106c2b961215864d1c2e703ee63269c25c4e80a577ffb2c1017b9c17dcdf83a1.
Original archive bytes and the normalized Cargo.toml are separately retained;
Cargo.toml.upstream preserves the latter. Original Cargo.lock is inert archive
material, not a new standalone execution scope. The product lock remains owner.

The three ingress changes bound IValueProvider SetValue UTF-16 conversion, reject
non-finite IRangeValueProvider SetValue arguments, and reject the unrepresentable
i32::MIN move magnitude. Both SetValue paths retain context, node and disabled
checks before value admission; text movement retains its existing range/context
guards. The portable value_input helper is the same source included
by the Windows adapter and the existing GUI test scope. There is no
standalone AccessKit compilation or test scope. No native run has been made.

The pointer reader relies on the COM/PCWSTR ABI caller to provide readable,
initialized, aligned, stable memory through a terminator or the bounded scan.
It cannot make an invalid pointer readable or avoid COM's prior marshaling cost.
The additional text-range correction uses the pinned windows-core 0.62.2
fallible cast_object_ref::<PlatformRange>() at Compare, CompareEndpoints and
MoveEndpointByRange, before any range-state access. A non-null interface alone
is not proof of the underlying Rust type. Null still returns E_INVALIDARG;
failed dynamic QueryInterface or an implementation-type mismatch returns its
COM error. Own-object identity, same-context checks and valid comparisons remain
unchanged. No raw pointer subtraction or unchecked implementation cast remains
at these three receiving sites.

The dynamic Any protocol trusts a valid COM object and a conforming generated
QueryInterface response. It does not validate arbitrary pointers or hostile ABI
implementations. A foreign provider or marshaled proxy that cannot expose the
local PlatformRange type may now fail; actual marshaling, apartment behavior and
native compatibility require their own qualified Windows controls. Initial COM
allocation, other native entry points and retained IME composition remain open.
This patch proves no complete native input security, Windows qualification or
permission to distribute the package.

Original MIT/Apache and Chromium BSD notices remain intact. LICENSE-MIT,
LICENSE-APACHE and LICENSE.chromium are retained from the same immutable
AccessKit revision because they were omitted from the published crate archive.
The MIT option is selected for original dual-licensed material; Chromium's
BSD-3-Clause notice is additional. The new original value_input.rs helper is
GPL-3.0-only and the unmodified GPLv3 text is retained in LICENSE-GPL-3.0-only.
The composite Cargo license records these distinct components; it does not
relicense third-party material or withdraw the original Apache alternative.
