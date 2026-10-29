//! Bounded standard-control calls; the live parent owns every child HWND.

use std::ptr;

use windows_sys::Win32::Foundation::{GetLastError, HINSTANCE, HWND, RECT, SetLastError};
use windows_sys::Win32::Graphics::Gdi::{DEFAULT_GUI_FONT, GetStockObject};
use windows_sys::Win32::UI::Controls::EM_LIMITTEXT;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BS_GROUPBOX, CreateWindowExW, ES_AUTOHSCROLL, GetClientRect, GetWindowTextLengthW,
    GetWindowTextW, LB_ADDSTRING, LB_GETCURSEL, LB_RESETCONTENT, LB_SETCURSEL, LBS_NOTIFY,
    MoveWindow, SW_HIDE, SW_SHOW, SendMessageW, SetWindowTextW, ShowWindow, WM_SETFONT, WS_BORDER,
    WS_CHILD, WS_DISABLED, WS_TABSTOP, WS_VISIBLE, WS_VSCROLL,
};
use windows_sys::core::w;

use super::view::Page;
use super::{NativeError, pointer_parameter};

/// Maximum text admitted from one native edit, before shared field validation.
const EDIT_CAPACITY: usize = 512;

/// Maximum display text in this fixed fixture; prevents accidental unbounded UI.
const DISPLAY_CAPACITY: usize = 4096;

/// Documented `SS_NOPREFIX` ABI value, also present in windows-sys 0.61.2's
/// `SystemServices` namespace. Display-only labels preserve literal ampersands;
/// importing that unrelated binding namespace is unnecessary for this constant.
const STATIC_NO_PREFIX: u32 = 0x0000_0080;

/// Stable local control identities, also used by the owned observation harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub(super) enum Id {
    /// Persistent application name.
    Brand = 1,
    /// Persistent sample-data notice.
    Banner = 2,
    /// Status navigation.
    Status = 101,
    /// Sources navigation.
    Sources = 102,
    /// Settings navigation.
    Settings = 103,
    /// Status page heading.
    StatusText = 110,
    /// Disabled synchronization action.
    Sync = 111,
    /// Explanation for disabled synchronization.
    SyncReason = 112,
    /// Explicitly illustrative trust profile.
    StatusProfile = 113,
    /// Plain-language headline with sample-state qualification.
    StatusHeadline = 114,
    /// Fixed sample UTC timestamp, unrelated to the host clock.
    SampleTime = 115,
    /// Visual grouping for the sample estimated offset.
    OffsetGroup = 116,
    /// Sample offset, units and public sign interpretation.
    OffsetValue = 117,
    /// Visual grouping for the sample uncertainty.
    UncertaintyGroup = 118,
    /// Illustrative uncertainty with explicit units.
    UncertaintyValue = 119,
    /// Visual grouping for independently labelled sample evidence.
    EvidenceGroup = 120,
    /// Separate sample authentication, agreement and applied-effect facts.
    EvidenceText = 121,
    /// Local navigation from Status to Sources; performs no measurement.
    ViewSources = 122,
    /// Stable source list.
    SourceList = 201,
    /// Selected source's current preview.
    Details = 202,
    /// Visual grouping and heading for the local editor.
    EditorTitle = 203,
    /// Display-name label.
    LabelCaption = 204,
    /// Hostname label.
    HostCaption = 205,
    /// NTS-KE port label.
    PortCaption = 206,
    /// Sources page heading.
    SourcesTitle = 207,
    /// Sources page's explicitly illustrative profile.
    SourcesProfile = 208,
    /// Visual grouping for the selected source details.
    DetailsGroup = 209,
    /// Display-name input.
    Label = 211,
    /// Hostname input.
    Host = 212,
    /// Port input.
    Port = 213,
    /// Checked local update action.
    Update = 220,
    /// Discard pending control text.
    Cancel = 221,
    /// Bounded validation or success message.
    EditorMessage = 230,
    /// Persistent local-editor lifetime explanation.
    EditorHint = 231,
    /// Settings page heading.
    SettingsText = 301,
    /// Disabled machine-policy action.
    Policy = 302,
    /// Distinguishes illustrative settings from installed machine policy.
    SettingsIntro = 303,
    /// Visual grouping for illustrative time-source trust settings.
    TrustGroup = 304,
    /// Read-only sample trust and time-scale settings.
    TrustText = 305,
    /// Visual grouping for unavailable clock-correction settings.
    CorrectionGroup = 306,
    /// Explains that no correction or administrator request is possible.
    CorrectionText = 307,
    /// Visual grouping for read-only prototype preferences.
    PreferencesGroup = 308,
    /// Describes inherited appearance and transient preview storage.
    PreferencesText = 309,
    /// Explanation adjacent to the disabled machine-policy action.
    PolicyReason = 310,
}

impl Id {
    /// Encode the declared repr(u16) fieldless identifier without truncation.
    #[expect(
        clippy::as_conversions,
        reason = "Fieldless repr(u16) enum values fit the identical unsigned representation"
    )]
    pub(super) const fn value(self) -> u16 {
        self as u16
    }
}

/// Which inherited standard control to construct; no custom subclass is used.
#[derive(Clone, Copy)]
pub(super) enum Widget {
    /// Read-only static text.
    Label,
    /// Standard pushbutton.
    Button,
    /// Visible but unavailable protected action.
    DisabledButton,
    /// Standard visual group; does not take focus or own sibling controls.
    GroupBox,
    /// One-line native edit with bounded input.
    Edit,
    /// Stable single-selection source list.
    List,
}

/// Non-owning child handle; its parent destroys it during full/partial teardown.
#[derive(Clone, Copy)]
pub(super) struct Control {
    /// Local checked identifier.
    pub(super) id: Id,
    /// Page association; None means persistent navigation/banner.
    pub(super) page: Option<Page>,
    /// HWND borrowed from the parent window on the same UI thread.
    pub(super) handle: HWND,
}

impl Control {
    /// Create one bounded child. The parent retains it even if later setup fails.
    ///
    /// # Errors
    /// Returns invalid text/style, unavailable system font or child creation failure.
    fn create(
        parent: HWND,
        instance: HINSTANCE,
        id: Id,
        page: Option<Page>,
        widget: Widget,
        text: &str,
    ) -> Result<Self, NativeError> {
        let text = wide(text)?;
        let base = WS_CHILD | WS_VISIBLE;
        let (class, style) = match widget {
            Widget::Label => (w!("STATIC"), base | STATIC_NO_PREFIX),
            Widget::Button => (w!("BUTTON"), base | WS_TABSTOP),
            Widget::DisabledButton => (w!("BUTTON"), base | WS_DISABLED),
            Widget::GroupBox => (w!("BUTTON"), base | unsigned(BS_GROUPBOX)?),
            Widget::Edit => (
                w!("EDIT"),
                base | WS_TABSTOP | WS_BORDER | unsigned(ES_AUTOHSCROLL)?,
            ),
            Widget::List => (
                w!("LISTBOX"),
                base | WS_TABSTOP | WS_BORDER | WS_VSCROLL | unsigned(LBS_NOTIFY)?,
            ),
        };
        // SAFETY: Parent/module are live on this UI thread, strings stay live
        // for synchronous creation; child hMenu is the nonpointer numeric ID.
        let handle = unsafe {
            CreateWindowExW(
                0,
                class,
                text.as_ptr(),
                style,
                0,
                0,
                1,
                1,
                parent,
                ptr::without_provenance_mut(usize::from(id.value())),
                instance,
                ptr::null(),
            )
        };
        if handle.is_null() {
            return Err(NativeError::last("CreateWindowExW(child)"));
        }
        // SAFETY: Borrowed system font; it is never deleted or replaced by us.
        let font = unsafe { GetStockObject(DEFAULT_GUI_FONT) };
        if font.is_null() {
            return Err(NativeError::Contract("Default native font is unavailable."));
        }
        // SAFETY: Standard control borrows the system-owned font; no ownership
        // transfer occurs and LPARAM is the documented redraw flag.
        unsafe { SendMessageW(handle, WM_SETFONT, font.expose_provenance(), 0) };
        if matches!(widget, Widget::Edit) {
            // SAFETY: Standard edit receives a finite UTF-16 input limit. Reads
            // independently bound programmatic WM_SETTEXT, which can bypass it.
            unsafe { SendMessageW(handle, EM_LIMITTEXT, EDIT_CAPACITY, 0) };
        }
        Ok(Self { id, page, handle })
    }

    /// Set checked bounded display text; embedded NUL is rejected, not truncated.
    ///
    /// # Errors
    /// Returns a display-budget/NUL violation or the native text-setting failure.
    pub(super) fn set_text(self, text: &str) -> Result<(), NativeError> {
        let text = wide(text)?;
        // SAFETY: This child belongs to the live parent; UTF-16 is terminated
        // and borrowed only for the synchronous call.
        if unsafe { SetWindowTextW(self.handle, text.as_ptr()) } == 0 {
            return Err(NativeError::last("SetWindowTextW"));
        }
        Ok(())
    }

    /// Read bounded exact UTF-16 control text; the model still owns field rules.
    ///
    /// # Errors
    /// Distinguishes excessive text, invalid UTF-16 and actual native read failure.
    pub(super) fn input(self) -> Result<String, InputError> {
        // SAFETY: Clear this thread's error slot for GetWindowTextLengthW's zero.
        unsafe { SetLastError(0) };
        // SAFETY: Own live edit HWND, invoked on its creation thread.
        let length = unsafe { GetWindowTextLengthW(self.handle) };
        // SAFETY: Capture the defined error immediately after the length query.
        let code = unsafe { GetLastError() };
        if length == 0 && code != 0 {
            return Err(InputError::Native(NativeError::Win32 {
                operation: "GetWindowTextLengthW",
                code,
            }));
        }
        let length = usize::try_from(length).map_err(|_| InputError::TooLong)?;
        if length > EDIT_CAPACITY {
            return Err(InputError::TooLong);
        }
        let mut buffer = [0_u16; EDIT_CAPACITY + 1];
        let capacity = i32::try_from(buffer.len()).map_err(|_| InputError::TooLong)?;
        // SAFETY: Clear only the thread-local last error before ambiguous zero.
        unsafe { SetLastError(0) };
        // SAFETY: Buffer is writable for capacity UTF-16 elements; the API
        // includes the terminator in that capacity and never writes past it.
        let count = unsafe { GetWindowTextW(self.handle, buffer.as_mut_ptr(), capacity) };
        // SAFETY: Immediate capture; zero count can legitimately mean empty text.
        let code = unsafe { GetLastError() };
        if count == 0 && code != 0 {
            return Err(InputError::Native(NativeError::Win32 {
                operation: "GetWindowTextW",
                code,
            }));
        }
        let count = usize::try_from(count).map_err(|_| InputError::TooLong)?;
        let text = buffer.get(..count).ok_or(InputError::TooLong)?;
        String::from_utf16(text).map_err(|_| InputError::InvalidUnicode)
    }

    /// Place a live child in a nonnegative client rectangle and check failure.
    ///
    /// # Errors
    /// Returns invalid geometry or a failed native move, without a fallback size.
    pub(super) fn place(self, [x, y, width, height]: [i32; 4]) -> Result<(), NativeError> {
        if x < 0 || y < 0 || width < 1 || height < 1 {
            return Err(NativeError::Contract("Invalid native control rectangle."));
        }
        // SAFETY: Live owned child, finite coordinates, synchronous repaint.
        if unsafe { MoveWindow(self.handle, x, y, width, height, 1) } == 0 {
            return Err(NativeError::last("MoveWindow"));
        }
        Ok(())
    }

    /// Change only this child's visibility; prior visibility is informational.
    pub(super) fn show(self, visible: bool) {
        // SAFETY: Parent still owns the standard child on this thread.
        unsafe { ShowWindow(self.handle, if visible { SW_SHOW } else { SW_HIDE }) };
    }

    /// Replace the stable source labels and restore the checked selected index.
    ///
    /// # Errors
    /// Returns invalid display text or native row-allocation/selection failure.
    pub(super) fn set_sources(self, labels: &[String], selected: usize) -> Result<(), NativeError> {
        // SAFETY: This is the standard listbox; it owns copied strings, not data.
        unsafe { SendMessageW(self.handle, LB_RESETCONTENT, 0, 0) };
        for label in labels {
            let text = wide(label)?;
            // SAFETY: LB_ADDSTRING copies the bounded terminated string during
            // the call; this listbox has neither owner-draw nor custom item data.
            let result = unsafe {
                SendMessageW(
                    self.handle,
                    LB_ADDSTRING,
                    0,
                    pointer_parameter(text.as_ptr()),
                )
            };
            if result < 0 {
                return Err(NativeError::Contract(
                    "Native source list could not retain its fixture rows.",
                ));
            }
        }
        // SAFETY: Selected index comes from the nonempty model, whose labels
        // were just copied in their original order.
        let result = unsafe { SendMessageW(self.handle, LB_SETCURSEL, selected, 0) };
        if result < 0 {
            return Err(NativeError::Contract("Native source selection failed."));
        }
        Ok(())
    }

    /// Return the native selected row; callers recheck against the model size.
    ///
    /// # Errors
    /// Returns an absent or invalid selection instead of inventing a row.
    pub(super) fn selection(self) -> Result<usize, NativeError> {
        // SAFETY: Standard listbox query takes no pointers or retained values.
        let result = unsafe { SendMessageW(self.handle, LB_GETCURSEL, 0, 0) };
        usize::try_from(result).map_err(|_| NativeError::Contract("No source row is selected."))
    }
}

/// Raw-control rejection, distinct from shared semantic validation failures.
pub(super) enum InputError {
    /// Native input exceeded the fixed transport allocation/read budget.
    TooLong,
    /// UTF-16 contained an unpaired surrogate, never normalized permissively.
    InvalidUnicode,
    /// Native retrieval failed rather than supplying user data.
    Native(NativeError),
}

/// Fixed child inventory. HWND values are borrowed; only the parent destroys them.
pub(super) struct Controls {
    /// Creation-order inventory, bounded by the fixed view specification.
    pub(super) items: Vec<Control>,
}

impl Controls {
    /// Add one fixed specification; failed setup leaves child custody in parent.
    ///
    /// # Errors
    /// Propagates checked text, style, font or child-window creation failure.
    pub(super) fn add(
        &mut self,
        parent: HWND,
        instance: HINSTANCE,
        id: Id,
        page: Option<Page>,
        widget: Widget,
        text: &str,
    ) -> Result<(), NativeError> {
        self.items
            .push(Control::create(parent, instance, id, page, widget, text)?);
        Ok(())
    }

    /// Resolve an internal identifier; missing controls are fatal setup defects.
    ///
    /// # Errors
    /// Returns a missing identifier; no control or handle is fabricated.
    pub(super) fn get(&self, id: Id) -> Result<Control, NativeError> {
        self.items
            .iter()
            .find(|control| control.id == id)
            .copied()
            .ok_or(NativeError::Contract("Required native control is missing."))
    }
}

/// Encode a bounded string without embedded NUL; never substitutes invalid text.
///
/// # Errors
/// Returns a display-budget or embedded-NUL violation before allocation.
fn wide(text: &str) -> Result<Vec<u16>, NativeError> {
    if text.len() > DISPLAY_CAPACITY || text.contains('\0') {
        return Err(NativeError::Contract(
            "Native display text exceeds its contract.",
        ));
    }
    Ok(text.encode_utf16().chain(std::iter::once(0)).collect())
}

/// Convert nonnegative native style constants without implicit signed truncation.
///
/// # Errors
/// Returns a negative constant instead of reinterpreting its sign bit.
fn unsigned(value: i32) -> Result<u32, NativeError> {
    u32::try_from(value).map_err(|_| NativeError::Contract("Invalid native style constant."))
}

/// Read current client dimensions, preserving minimized zero size as a no-op.
///
/// # Errors
/// Returns native rectangle failure, arithmetic overflow or negative dimensions.
pub(super) fn client_size(window: HWND) -> Result<(i32, i32), NativeError> {
    let mut rectangle = RECT::default();
    // SAFETY: Live original HWND and writable initialized RECT on this thread.
    if unsafe { GetClientRect(window, &raw mut rectangle) } == 0 {
        return Err(NativeError::last("GetClientRect"));
    }
    let width = rectangle
        .right
        .checked_sub(rectangle.left)
        .ok_or(NativeError::Contract("Client width overflow."))?;
    let height = rectangle
        .bottom
        .checked_sub(rectangle.top)
        .ok_or(NativeError::Contract("Client height overflow."))?;
    if width < 0 || height < 0 {
        return Err(NativeError::Contract("Negative native client size."));
    }
    Ok((width, height))
}
