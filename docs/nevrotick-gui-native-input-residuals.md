# Native GUI input residuals at raw-input integration

This is a source-only qualification record. It introduces no dependency, toolkit
profile, native adapter, graph admission, operating-system effect or permission.
The Rust raw owner limits retained admitted editor content; it does not establish
zero initial OS allocation or a whole-process memory bound. No native clipboard,
IME, UIA, focus/timing, pointer menu or Windows service behavior was executed by
this audit. Fresh current-cohort verification receipts remain the authority for
any subsequent outcome.

1. **Clipboard and native input allocation.** Stock LineEdit/TextInput may already
   allocate or convert incoming paste/UIA text before the Rust raw callback. The
   receiver bounds content before retaining its own accepted String and returns
   the entire prior raw value on rejection; that rollback cannot undo an earlier
   OS/toolkit allocation.
2. **IME retention and initial allocation.** Pinned TextInput can replace
   retained `preedit_text` without edited. The later admitted Windows-only
   `unstable-winit-030` public event filter now bounds Preedit/Commit before
   Slint retention:1024 original UTF8 bytes, printable ASCII, and both cursor
   offsets within the string. Empty/cancel and reversed selections remain valid;
   byte-bounded ordinary keyboard chunks preserve control/private-key semantics.
   Explicit Winit/software selection and the pre-show/pre-observer installation
   keep the maintained production path from silently using another adapter.
   Twelve pure predicate tests and14 source controls are distinct from actual
   native behavior. The filter cannot undo Winit/OS allocation or the derived
   modifier Key clone; cancellation, focus, queued events and native lifetime
   still need Windows qualification. Raw callbacks alone never supplied this
   additional composition boundary.
3. **Right-pointer menu opening.** Public ContextMenuArea show cannot be overridden
   for a fresh private Root permission check on each right-button open. The native
   event checks current enabled; every menu activation checks private current
   field/action permission before clipboard/history effects. Keyboard Menu/ShiftF10
   OpenMenu6 does have a fresh Root check. These are different guarantees.
4. **Already-open Windows menu.** Public ContextMenu.close closes its tracked Slint
   fallback popup. The native Windows Muda adapter owns separate menu state, and
   native-success generation skips the fallback popup ID. Its native dismissal
   on disable is not qualified. Activation permission remains independent of
   successful dismissal.
5. **Portable selection clipboard.** Linux selection left-release copy and
   middle-release paste lie outside the default menu/shortcut hook. Pinned Winit
   Windows selection clipboard is a no-op; this observation is not a Windows
   alternate clipboard path.
6. **AccessKit Windows boundary.** The later GUI27 admitted source derivative
   bounds PCWSTR conversion, rejects nonfinite numeric values and checks movement
   negation. Eight portable tests execute the same source helper. This does not
   qualify initial COM allocations, pointer readability, Windows dispatch or
   native accessibility behavior. A later source correction replaces the three
   unchecked foreign ITextRangeProvider casts with fallible
   `cast_object_ref::<PlatformRange>()` before state access. Null and failed
   queries reject; own-object identity/context/lock order stays unchanged. The
   existing eight value-helper tests do not exercise this new COM path. Dynamic
   casting still assumes valid conforming COM objects and cannot bound a foreign
   query's duration; unsupported marshaled proxies may reject. Actual provider,
   proxy, apartment, reference-count and native compatibility checks remain open.
7. **Geometry.** Scroll coordinates remain a two-way presentation sink. Recovery
   captures finite bounded coordinates before retaining/restoring them; that
   check is not proof that arbitrary raw generated setter floats never reach the
   toolkit/renderer first.

Required native observations include current-context denial before every clipboard
read/write/history operation, queued menu activation after page/phase/modal change,
composite Menu/F10 press+release, true pointer selection/focus, rejection rollback
with equal parent value, native accessibility SetValue, retained IME composition,
menu disable/dismissal, and safe terminal Quit with tray preference enabled. A
receiver-only boolean permission fixture cannot establish actual OS I/O absence.

## Later native source integration: behavior still unqualified

The current Windows source implements tray icon/menu activation and an explicit
Test notification adapter with its real host/receiver chain. Text contract2 has
509 active IDs, 596 stable ordinals and426 immutable output rows. The six tray
commands remain code-owned; private current interaction guards precede dispatch.
Test submission separately checks accepted preferences, current observer and
fresh Windows permission. A saved preference alone establishes no readiness.
OS acceptance establishes neither a visible banner nor reading nor any live
service/clock state. Production incident, source/live-service/clock and protected
Apply adapters remain disconnected; sign-in registration remains unavailable.

The running-toast WinRT activation callback supplies untrusted UTF-16 arguments
through COM. `Arguments()` can allocate before the fixed54-unit parser, which validates a closed token without
copying arbitrary text. One bounded event slot reaches the UI owner; current
session/notice/attempt and interaction checks select only code-owned navigation,
owned-dialog focus or ignore. Cold-start activation is not implemented. This
source path needs actual Windows COM/apartment, callback lifetime, queued event,
stop/cleanup and activation tests; it closes none of residuals1–7 above.

The UI-only exit source waits for actual owned native cleanup; timeout or failure
keeps the app open, and no service/clock change follows. Menu disable/re-enable,
TaskbarCreated, late callbacks and safe emergency Quit remain native observations,
not results established by portable receiver or source-coverage controls.

The GUI27 portable receipt and candidate identity are recorded in
[the active task](../tasks/s0.5-ui-comparison.md#gui27-integrated-portable-checkpoint).
That receipt covers text contract1 before these adapters. The current native/
text-2 cohort requires its own integrated configured and native evidence; this
record transfers no historical pass. The production semantic approval ledger
remains empty, and technical/source checks grant no human language approval.
