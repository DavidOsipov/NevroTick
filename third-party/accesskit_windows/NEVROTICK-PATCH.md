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
standalone AccessKit Cargo package scope. The later subclass-module facade below
is a separate std-only control, with no Linux AccessKit backend. No native run
has been made.

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

The range-comparison repair prevents recursive acquisition of one range's state
read lock and inconsistent acquisition of distinct owners. Compare and the
same-object CompareEndpoints branch use range_state's shared read-pair helper:
identical locks supply one guard to both projections; distinct locks are acquired
in increasing live-owner-address order and retain both guards until comparison
finishes. The comparison still receives its original left/right operands. The
addresses select only a lock order during these borrows, not COM or persistent
identity. This preserves existing context/type checks, endpoint errors and the
simultaneous distinct-range snapshot. It avoids both a queued writer blocking a
recursive read and opposing comparisons retaining different first read guards
while queued writers exclude their second reads. Portable controls include this
exact helper in the existing GUI test scope, real queued writers, bounded
second-read probes, reversed operand order and poison/unwind guard release. A
forced blocked probe may reject the comparison; it does not establish successful
concurrent comparison under writer contention. These controls do not execute COM,
qualify Windows scheduling or lift native holds.

Original MIT/Apache and Chromium BSD notices remain intact. LICENSE-MIT,
LICENSE-APACHE and LICENSE.chromium are retained from the same immutable
AccessKit revision because they were omitted from the published crate archive.
The MIT option is selected for original dual-licensed material; Chromium's
BSD-3-Clause notice is additional. The new original value_input.rs and
range_state.rs helpers are
GPL-3.0-only and the unmodified GPLv3 text is retained in LICENSE-GPL-3.0-only.
The composite Cargo license records these distinct components; it does not
relicense third-party material or withdraw the original Apache alternative.

## Subclass callback custody correction — 9 November 2026

The selected accesskit_winit 0.33.2 Windows adapter calls SubclassingAdapter::new,
so this is a selected dependency path, not an unused example. The original
AccessKitAdapter HWND property could select a raw Rust address, and the original
numeric WM_NCDESTROY flag could suppress uninstall while its callback remained
installed. Microsoft documents that equal-integrity peers can call SetProp and
SendMessage against the window; native peer-process reproduction remains open.

The callback now selects only a registration in its owner thread's process-owned
registry. A cloned registration plus an upgraded Rc retains the actual adapter
through the complete current callback, including reentrant owner Drop. HWND
properties supply no pointer, registration identity or lifetime fact. Rc makes
the public adapter thread-affine; construction checks the native HWND owner
thread before installing, using the additional Win32_System_Threading feature
of the existing windows dependency. No package version/dependency is added.

Uninstall always considers the actual chain rather than a message-derived
terminal flag. It removes only its exact Rc registration generation and restores
the predecessor only when its procedure is the current head. WM_NCDESTROY can
prompt that same checked detach; the message does not prove window destruction.
A forged message may therefore detach accessibility from a live window, but it
cannot justify freeing an installed callback target. Updates after a successful
detach return no events.

When a later in-process subclass still owns the chain head, Drop deactivates and
releases the adapter/handlers while retaining only forwarding metadata and a
Weak control block. This avoids clobbering the later procedure or leaving a
freed callback target. Only trusted adapter construction inserts registry entries;
peer messages insert none. Retired forwarding records are limited by those owned
window chains and are pruned after an owner-thread observation of an invalid
HWND, or released with the owner thread. In-flight callbacks retain their own
Rc until return. A still-registered HWND, including ambiguous handle reuse
beneath a later subclass, refuses another installation rather than replacing its
predecessor or letting an old owner remove a new generation. This does not claim
immediate retirement of another subclass's chain or proven native cleanup.

The GPL-3.0-only tests/subclass_lifecycle_harness.rs includes this exact source
module with inert windows/accesskit facades. Its paired controls distinguish
wrong-owner selection through another live object's property and skipped teardown
after a forged destroy, without invoking a freed pointer. Additional controls
cover normal focus/drop, later-subclass forwarding, duplicate construction,
invalid HWNDs, detach/reuse generations, owner-thread checks and reentrant Drop.
Those controls exercise Rust registration/lifetime logic, not USER32, UIPI,
COM/UIA, window-handle generation or Windows scheduling. Configured compiler,
source-policy and native-memory/native-UI qualification remain coordinator-owned;
no native result or broader accessibility pass follows from preparing the harness.

Primary contracts: [SetPropW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setpropw),
[SendMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew),
[SetWindowLongPtrW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw),
and [subclass chaining](https://learn.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
