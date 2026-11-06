# Current GUI input inventory and receiver map

This source-only audit covers the GUI-M0 raw/typed-copy, grouped Settings,
SourceVisual/Search and Emergency UI integration candidate. The historical
baseline declared54 AppWindow callbacks; this candidate declares61. Its private
review handoff binds the exact declaration inventory and candidate source
identities. No compiler, Cargo, application, native, network, installation or
dependency operation was performed by this audit. Private admitted rustfmt and
artifact-application checks are source/format checks only. The integrating
coordinator owns fresh current-cohort compilation, executed tests, visual checks
and native admission; historical static-GUI passes do not qualify this cohort.

Current source update: text contract **2**, catalog schema **2**, English revision
**2** and informational GUI build **2** are integrated, with **509 active IDs**,
**596 stable ordinals** and **426 immutable output rows**. Windows tray and explicit
test-notification host/adapter source is implemented; its native behavior remains
unqualified. The 61 AppWindow callbacks are unchanged; native OS callbacks are
additional ingress, described below. The GUI27 evidence at the end is historical
text-contract-1 evidence and does not qualify the later native/text-2 cohort.
The production semantic approval ledger remains empty.

Main source surfaces: `crates/nevrotick-ui/src/ui_drafts.rs`,
`src/slint_host.rs`, `src/slint_host/{raw,source,inspection,policy,recovery,shell}_workflow.rs`
and `crates/nevrotick-ui/ui/{app,sources,source-editor,settings,recovery,policy-panel}.slint`.
The later native source adds `src/slint_host/{native,native_notification}_workflow.rs`,
`src/native_tray.rs`, `src/native_tray/windows.rs`,
`src/notifications_ui/native_delivery.rs` and `src/native_notifications_windows.rs`.
The raw integration artifact is SHA256
`85e675b389bb896aca39fddebaeb222feb00650567bd8854a642490fa4e6f01d`;
this hash identifies a reviewed input artifact, not an integrated build result.

All input is untrusted by default. Valid UTF-8 alone grants no size, alphabet,
schema, current-context, identity, revision or effect authority. A visible or
disabled Slint control, translated label, public generated setter, authenticated
transport or local origin likewise supplies no such authority. Public generated
projection properties are deliberately spoofed by tests; receivers read private
owners and actual workflow state instead.

## Field and choice surfaces

| Surface and original request | Admission before retained owner allocation/work | Actual owner/context and failure behavior |
| --- | --- | --- |
| Source label, host, port raw edits (field IDs0/1/2) | UiDrafts checks original byte length before allocating accepted String; printable ASCII U+0020..U+007E only; label/host1024B each, port64B; empty and format-invalid ASCII preserved | Actual Power/Sources plus private active Add/Edit; no Quit/Recovery-leave/exit/projecting/catalog/raw failure. Rejected character/oversize/field ID retains complete previous raw text; sender receives that value and explicit revision reprojects all raw fields. Source fields are parsed only at matching Submit. |
| Query raw edit (ID3) | Original1024B/ASCII raw cap, no trim/truncation; accepted query independently max253B printable ASCII, empty allowed, significant spaces retained | Actual Power/Sources/list, accepted SearchVisibility expanded, at least one checked source. Details/Review/editor excludes list. Matching valid Submit changes only query/visible projection and collapses Search; invalid retains previous accepted query plus bounded invalid raw draft. |
| Custom minutes raw edit (ID4) | Original64B/ASCII raw cap, including malformed `0`, `1.5`, sign or ASCII word; Submit independently max20B ASCII decimal → NonZeroU64 | Actual Settings/Sync0 and retained raw choiceCustom4. ChoicesAutomatic0/2min1/5min2/15min3/Custom4 are closed Rust raw state. Save must match choice and active custom text; unused preset text is inert. Invalid/zero/fraction/overflow keeps accepted interval and raw. |
| Helper hosts raw edits (IDs5/6) | Original1024B printable ASCII each; accepted used hosts share max253B host grammar/DNS-label<=63/IP checks | Actual Power/Recovery/Editing; first only for count>=1, second only count2. Used count/fields must match private snapshot at Submit. Count0/1 unused args are never copied into any owner, validated as active data or treated as auth evidence. Earlier legally entered unused raw text may remain bounded. |
| Raw selectors interval/helper/protocol/profile (IDs0..3) | Signed identity decode; interval0..4, count0..2, protocol0/1, profile0..2; no integer indexing or label-derived choices | Each checks actual page/section/phase. ProtocolPortDraft alone owns protocol and monotone manual-port provenance; RawEditPort marks manual only after accepted raw edit. Default-port switch is prepared before candidate publication. Wrong context/invalid option retains all raw and accepted state and returns current numeric choice. |
| Notifications/tray/sign-in bool requests | bool ABI is closed, but current context/prerequisite remains mandatory | Notifications only Settings/Notifications2; tray/sign-in only Preferences4. Sign-in requires accepted tray preference; disabling tray clears sign-in. Each returns actual accepted bool even on rejection. The native host consumes accepted tray preferences, while actual readiness stays a separate private observation. Sign-in registration remains unavailable; no service/clock authority follows. |
| Diagnostic view/scope/duration | Actual Power/Diagnostics; view0..3, collection only actual TemporaryCollection2, scope0..2/duration0..2 tuple checked atomically | Projected numeric values are display only. Wrong route/modal/tuple retains private choices. No file/logging/collection/clock adapter. Dormant diagnostic-action-requested(int) ignores its ID and returns fixed unavailable feedback; keep disconnected/remove before a real adapter. |
| Search expansion | Closed bool → private SearchVisibility enum; no data from `checked`/error strings decides panel ownership | Actual Power/Sources/list; opening requires nonempty checked source owner. Guarded toggle uses explicit callback response and display-only confirmed mirror. Navigation retains query/raw expansion; Details/Review/empty state cannot admit hidden query input. |

The seven raw fields retain at most **5248 content bytes** (five1024B fields and
two64B fields), plus fixed closed selectors. Transactional internal reset checks
all supplied groups, including unused text, before committing any field; prepared
candidate content is at most another5248B. This does not bound allocator metadata,
caller/widget strings, OS/COM input or total process memory. Redacted Debug and
fixed typed error messages never include rejected text. An internal reset
invariant failure closes private RawAdmission; ordinary invalid user input does
not set this terminal flag. Existing host readiness stays available for safe
emergency Quit/Cancel/Stay.

## Callback, model and workflow inputs

- The final callback manifest accounts for all61 declarations. Five new raw/menu/
  search receivers and the two current Status receivers supplement the historical
  54. Raw text/choice return values are retained snapshots, not acknowledgements.
- SourceSelected/Details/Remove/Undo accept stable positive checked IDs, never a
  filtered row index. SourceDrafts retains at most16 effective sources/32 deltas;
  IDs are checked i32 and not recycled. Duplicate host+protocol identity, DNS/IP
  normalization and capacity/membership checks precede accepted mutation.
- Source Add/Edit/Select/Remove require the actual list; Details and dedicated
  Review exclude hidden list controls. Undo/Discard require actual Review;
  opening an empty Review rejects. Failure leaves deltas/selection/accepted data.
  Legitimate Undo identity conflict remains visible in Review with focused reason.
- Clear query has an independently checked visible control: expanded Search with
  nonempty source owner, or collapsed nonempty accepted-query summary. Collapsed
  Search with only unsubmitted raw text cannot erase that hidden draft. Duplicate
  CloseDetails is inert and cannot erase another validation message.
- Every Submit independently matches the active private raw snapshot before
  domain mutation: all source fields+protocol, used helper fields+count,
  interval choice+active custom text, profile choice, query. A well-formed but
  mismatching payload gets fixed RAW_STALE, restores the raw projection and keeps
  accepted state. Matching malformed ASCII still reaches the domain validator;
  early alphabet rejection is not mislabeled as a domain parse test.
- Policy Summary/Editing/Review phases are private. NTS, explicit plainNTP and
  observe-only are separate previews. No auto downgrade, mixed votes, Windows
  Time handover, activation or acquired observe-mode auth is inferred. Details
  expose checked fields and explicit NotObserved/NotMeasured/NotEstablished facts;
  plain NTP auth/cert facts are NotApplicable.
- Navigation Page/Mode and Settings section0..4 are closed generated/checked
  types. They go through the shared Recovery-held destination, preserve raw and
  accepted previews on ordinary navigation, and cannot bypass an owned modal.
  Entering TimePolicy3 selects Power atomically; accepting Simple resets that
  advanced section toSync0 without losing its draft. Simple is complexity, not
  privilege. No input command becomes backend authority.
- Current source/changes/visible models are projections from those checked sole
  owners. Statistics retains the all-source model. Text/model flags/selection
  projection forgery cannot supply receiver identity, phase or permission.
  Generated Rust setters can still construct/render arbitrary models in-process;
  they are not an admitted external model transport or proof of bounded toolkit
  allocation. Any future adapter must retain the checked producer boundary.
- Status Check/Remedy have no raw action argument. Actual Status route is checked;
  status comes from explicit disconnected FixtureStatusPort, not a forged label,
  local preview, empty model or implied measurement freshness. Remedy is guarded
  navigation only. No fresh-check/clock/service request is submitted.

## Toolkit, keyboard, pointer and accessibility inputs

Seven GuardedLineEdits, six guarded ComboBoxes (including two diagnostic choices),
three guarded checkboxes and one guarded Search toggle remove data two-way
bindings. Geometry page-scroll-y remains the only two-way root binding. Raw
setters snapshot the bounded Rust owner before notifying the toolkit, hold no
RefCell borrow across setters and suppress reentrant ordinary transitions.
LineEdit assigns the receiver's entire return after every edited/UIA SetValue,
and revision forces resync even when the parent's string compares equal. No
silent truncation or lossy character replacement is used.

Every menu effect0Undo/1Redo/2Cut/3Copy/4Paste/5SelectAll calls current private
permission **before** the public LineEdit operation can touch clipboard/history.
Permission requires initialized/nonprojecting/nonfailed owner, modal-free actual
field presence and editable phase. Six titles are code-mapped RawCopy resources;
translated labels never select actions. Known Ctrl editing shortcuts are consumed
on denial before stock TextInput shortcut handling. Menu/ShiftF10 press and
release use borrowed `contains` classification matching the stock path; synthetic
text>1024B is consumed before the additional bounded scan. Keyboard OpenMenu6
checks current permission. This classifier cap does not bound initial native key
or clipboard string construction.

## Earlier native text admission

The maintained Windows `create_native` path explicitly selects the pinned
Winit/software backend before creating AppWindow, then installs the sole public
window event filter before native observers and show. Fixture `create()` keeps
its manual-platform/non-native contract. The stateless filter captures no root,
Window/Arc, raw draft or RefCell; no additional controller or worker is created.

- Borrowed keyboard `event.text` and logical Character chunks are bounded at1024
  original UTF8 bytes. The modifier-supplement Character is checked immediately
  after Winit returns its owned Key clone. This byte-only keyboard check preserves
  control, private and named-key semantics; final field alphabet,64/1024-byte
  limits, private ownership and Submit parsing remain with the raw receiver.
- IME Preedit and Commit require at most1024 original bytes and printable ASCII,
  including empty/spaces. Both optional cursor offsets must lie within that
  original string; inclusive and reversed selections remain valid. ASCII and
  the ceiling imply UTF8 boundaries and safe signed cursor representation.
- Rejection returns PreventDefault before AccessKit, SharedString conversion,
  cursor casts and retained TextInput composition. It does not trim, repair,
  normalize, allocate a replacement string or grant effect authority. Empty
  cancellation remains valid. All other lifecycle/focus/Close/IME enabled or
  disabled events propagate to their existing owner.

The exact12 predicate tests first observed7 missing-protection failures and5
valid passes, then12/12 GREEN under a guarded std-only compiler/test diagnostic.
Fourteen bounded source controls also check actual production entry, selection,
hook order, event mapping and forwarding. The frozen source controls run before
Cargo in each schema3 verification lane, with nonempty/unskipped accounting.
These are source/predicate results; current configured product and Windows
source receipts belong to the active task. No actual keyboard, IME cancellation,
focus, clipboard, UIA, native lifetime or complete process-memory qualification
follows. Earlier OS/Winit allocations and the derived Key clone precede the
corresponding application check.

Material source limitations remain explicit:

1. Stock clipboard/IME/native UIA can allocate or convert text before app
   admission. The later public Winit filter bounds retained composition without
   relying on edited or the LineEdit/FocusScope interface. It cannot undo the
   earlier OS/Winit allocation; actual native cancellation/focus behavior and
   other clipboard/COM ingress remain separately unqualified.
2. Public ContextMenuArea show cannot be overridden for fresh private permission
   on each right-pointer open. Native current `enabled` is checked and every
   activation rechecks Root; keyboard6 does have a fresh check. An overlay with
   TouchArea would break left selection/focus and was not introduced.
3. Public ContextMenu.close closes its tracked fallback popup; native Windows
   Muda menu state is separate. Already-open native dismissal is unqualified.
   Permission at activation does not depend on dismissal success.
4. Linux selection-clipboard left/middle release lies outside default menu/
   shortcut hooks. Pinned Winit Windows selection clipboard is a no-op; this is
   not described as a Windows alternate clipboard path.
5. AccessKit Windows PCWSTR-to-String unwrap and initial allocation were found
   upstream. The later GUI27 source derivative bounds conversion, nonfinite
   values and movement negation; its eight portable helper controls passed.
   The later three text-range receivers replace unchecked implementation casts
   with fallible exact-type admission before state access. Null and query/type
   failure reject; valid own-object bodies and context/lock ordering remain.
   Two source controls exercise16 known regression mutations. Initial COM
   allocation, conforming-ABI assumptions, foreign/proxy behavior and actual
   Windows dispatch still need native qualification; these controls do not
   establish it or bound the duration of a foreign query.

No native clipboard, IME, UIA, pointer, focus/timing or window-menu behavior was
executed by this audit. Direct permission receiver tests prove only their stated
admission state, not absence of OS clipboard I/O along every native path.

## Files, resources, catalogs, URIs, metadata and absent surfaces

- There is no reached file picker, drag/drop/file import, arbitrary URI field,
  arbitrary image/font/resource path, rich text/HTML/Markdown renderer, command
  string or shell evaluator. Source hosts are literal text; they are not resolved
  or fetched. No uploaded prototype ZIP is parsed by the runtime GUI.
- About's repository action uses one literal HTTPS GitHub URI. Rust permits only
  actual About plus initialized/idle/nonfailed/modal-free ownership; the label or
  a contributor string cannot substitute its scheme/host/path. A stale action
  from Status/Sources cannot request the browser. Repository browser opening is
  an explicit fixed-target action; the native observer facilities below have
  their own separate receiving owners.
- Images/resources are compile-time @image-url paths under maintained assets.
  Locale/source text is never reinterpreted as an import, asset path, expression,
  component, format string, callback, field ID, action ID, schema or authority.
  Static GUI resource existence and both-direction orphan/missing sink coverage
  belong to the locale agent's independent manifest checker, not this inventory.
- Catalog JSON/CBOR is a separate untrusted byte/file surface owned by the locale
  pipeline. The implemented delivered schema2/text-contract2 admits a closed schema,
  exact semantic fingerprint, locale/plural IDs, bounded raw/decoded strings,
  keys/counts/depth/nodes/work and fixed typed placeholder slots before complete
  copy publication. Raw JSON131072B, CBOR65536B, decoded65536B; individual text
  1024B/512 scalars; keys96B; fields2048; nodes4096; depth4; work4m; rendered text
  4096B/2048UTF16. The protected registry has 509 active IDs across 596 stable
  ordinals; the largest map remains capped at512 and the immutable UI snapshot
  has397 static plus29 feedback rows. Production human-review
  ledger is empty; preparing an unapproved review artifact grants no approval.
- GUI build version is code-owned metadata only. Exact text-contract version and
  semantic fingerprint determine catalog compatibility; symbol positions/layout
  paths are independently covered and not invented compatibility authority.
- Windows tray icon/menu activation has an actual host/timer/worker source
  chain. Six code-owned IDs0..5 reach the existing shell receiver, which restores
  the window and checks current private interaction ownership before dispatch.
  The two consumed menu-label IDs are active; eight other future tray meanings
  remain reserved. Accepted tray preference alone does not establish readiness.
  Native lifecycle, menu callbacks and teardown still require Windows evidence.
- Explicit Test notification has a guarded Settings/Notifications receiver and
  current-snapshot title/body/action preparation, then a Windows DOM/Show adapter.
  Accepted preferences, fresh Windows permission and current observer presence
  are independent prerequisites. Accepted/indeterminate results stay distinct;
  repeated attempts are coalesced. OS acceptance confirms neither banner nor
  reading, and the test establishes no clock/service/production-incident fact.
- Toast activation arguments are untrusted native UTF-16. The adapter's COM
  `Arguments()` call precedes the fixed54-unit parser; that parser does not bound
  initial OS/COM allocation. A one-entry channel carries only the checked token
  or fixed error to the UI owner. Contextual navigation requires current session/
  notice correlation; stale/invalid events can only select guarded Status,
  focus an owned dialog or be ignored. No route/command comes from token text.
  This running-observer path does not implement cold-start activation.
- Source/live-service/clock/protected Apply and production incident adapters
  remain disconnected; service/diagnostic effects stay unavailable. Service
  commands0..7 and notice identities are closed typed values. There is no active
  wire record, OS service command, external report parser, log file, exporter,
  scheduled task or rate/quorum/crypto/step control. UI-only exit waits for owned
  observer cleanup; closing to a ready tray keeps the observer running. Exit
  timeout/failure keeps the app open and requests no service or clock change.
  Source implementation is not native qualification.

## Repair and evidence boundaries

The private implementation addresses the reproduced source paths for offscreen
preferences/Cancel, actual About URI, source inline subflows, hidden Clear,
widget-to-raw mutation, mismatching Submit, clipboard effect ordering and safe
raw-terminal emergency admission. Source and parser tests author matching legal,
malformed, boundary/overbudget, wrong-context, stale projection and modal controls.
Their execution remains the fresh integrated lane. Historical failures and
historical capture receipts are not reused as evidence for this new cohort. Native
holds, disconnected backend effects and upstream ingress gaps stay open. Later
native/text-2 source integration does not transfer an earlier portable pass or
close the clipboard, IME, COM, UIA, menu or native lifecycle residuals.

## Later integrated portable checkpoint

GUI27 executes the combined raw-owner, versioned text, guarded widgets and
AccessKit value-helper source: **137 UI product tests and 17 doctests passed**.
The complete configured portable row also produced 47 actual software-renderer
frames. See [the active task](../tasks/s0.5-ui-comparison.md#gui27-integrated-portable-checkpoint)
for the exact 350-input receipt and remaining review work. The opening paragraphs
describe this audit's source-only origin; they do not supersede that later
executed evidence. Conversely, the later portable pass does not close any native
residual listed here.

The bounded `prepare_locale` CLI was separately executed against the same GUI27
inputs. Valid English JSON produced the exact 44955-byte CBOR review fixture;
truncated JSON, UTF-8 BOM, invalid UTF-8, unknown fields, emoji, 131073-byte input
and unexpected arguments all failed with zero stdout and fixed redacted stderr.
This checks producer I/O, not catalog activation or semantic approval. The
source-bound diagnostic receipt is `locale-cli-5xgq8h82/receipt.json`, SHA-256
`c9a03e1f004aa2c567de7be0e123a8a5e8ba679f53a8f797dc05f912c28c3963`.
