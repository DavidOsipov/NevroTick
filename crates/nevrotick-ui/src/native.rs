//! Unelevated fixture window and its single-threaded Win32 ownership boundary.
//!
//! The runner owns callback state until its original window is destroyed. Child
//! HWNDs belong to that window; callbacks never hold a model borrow across a
//! native call. No network, service, configuration or clock APIs are present.
#![expect(
    unsafe_code,
    reason = "Reviewed Win32 adapter; every unsafe operation documents its native contract"
)]

mod controls;
mod view;

use std::cell::{Cell, OnceCell, RefCell};
use std::ffi::c_void;
use std::fmt;
use std::ptr;

use windows_sys::Win32::Foundation::{
    GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, SetLastError, WPARAM,
};
use windows_sys::Win32::Graphics::Gdi::{COLOR_BTNFACE, GetSysColorBrush};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    GWLP_USERDATA, GetMessageW, GetWindowLongPtrW, IDC_ARROW, LoadCursorW, MINMAXINFO, MSG,
    PostQuitMessage, RegisterClassW, SW_SHOW, SetWindowLongPtrW, ShowWindow, TranslateMessage,
    UnregisterClassW, WM_COMMAND, WM_GETMINMAXINFO, WM_NCCREATE, WM_NCDESTROY, WM_SIZE, WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
};
use windows_sys::core::w;

use crate::model::PreviewModel;
use controls::Controls;
use view::Page;

/// Bounded startup, operating-system or adapter-contract failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeError {
    /// A documented Win32 failure with its immediately captured error code.
    Win32 {
        /// Static operation name; never raw user input.
        operation: &'static str,
        /// Native error code, including zero where the API supplies no detail.
        code: u32,
    },
    /// A checked invariant failed before further native effects.
    Contract(&'static str),
}

impl NativeError {
    /// Capture the current thread's last error immediately after a failed API.
    fn last(operation: &'static str) -> Self {
        // SAFETY: GetLastError has no pointer or handle preconditions.
        let code = unsafe { GetLastError() };
        Self::Win32 { operation, code }
    }
}

impl fmt::Display for NativeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Win32 { operation, code } => {
                write!(formatter, "{operation} failed (Windows error {code}).")
            }
            Self::Contract(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for NativeError {}

/// Stable callback storage, owned by `run`; interior mutations stay on its thread.
struct State {
    /// Original live HWND, cleared before final native destruction completes.
    window: Cell<HWND>,
    /// Checked transient fixture state; borrows end before any Win32 call.
    model: RefCell<PreviewModel>,
    /// Immutable child-handle inventory, initialized once before display.
    controls: OnceCell<Controls>,
    /// Currently visible presentation page.
    page: Cell<Page>,
    /// First recoverable callback failure, consumed by the message-loop owner.
    failure: Cell<Option<NativeError>>,
}

impl State {
    /// Create inert storage around a validated model; creates no native resource.
    fn new(model: PreviewModel) -> Self {
        Self {
            window: Cell::new(ptr::null_mut()),
            model: RefCell::new(model),
            controls: OnceCell::new(),
            page: Cell::new(Page::Status),
            failure: Cell::new(None),
        }
    }

    /// Retain the first callback failure and request ordinary loop teardown.
    fn fail(&self, error: NativeError) {
        if self.failure.get().is_none() {
            self.failure.set(Some(error));
        }
        // SAFETY: Posts only to this UI thread; no handles or borrowed buffers.
        unsafe { PostQuitMessage(1) };
    }
}

/// Registered class owns registration only; module, brush and cursor are borrowed.
struct WindowClass {
    /// Current executable module, valid for the entire process.
    instance: HINSTANCE,
    /// Whether this owner still owes an `UnregisterClassW` call.
    registered: bool,
}

impl WindowClass {
    /// Register the sole fixture class with borrowed standard system resources.
    ///
    /// # Errors
    /// Returns unavailable system resources or a native registration failure.
    fn register() -> Result<Self, NativeError> {
        // SAFETY: Null requests the current executable's borrowed module handle.
        let instance = unsafe { GetModuleHandleW(ptr::null()) };
        if instance.is_null() {
            return Err(NativeError::last("GetModuleHandleW"));
        }
        // SAFETY: IDC_ARROW is a documented shared cursor identifier.
        let cursor = unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) };
        if cursor.is_null() {
            return Err(NativeError::last("LoadCursorW"));
        }
        // SAFETY: COLOR_BTNFACE is a valid system colour; the brush is borrowed.
        let background = unsafe { GetSysColorBrush(COLOR_BTNFACE) };
        if background.is_null() {
            return Err(NativeError::Contract("System window brush is unavailable."));
        }
        let class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            hCursor: cursor,
            hbrBackground: background,
            lpszClassName: w!("NevroTick.UI-M0"),
            ..WNDCLASSW::default()
        };
        // SAFETY: Structure, class string and callback remain valid for the call
        // and registration. Registration is local to the current module.
        if unsafe { RegisterClassW(&raw const class) } == 0 {
            return Err(NativeError::last("RegisterClassW"));
        }
        Ok(Self {
            instance,
            registered: true,
        })
    }

    /// Unregister after all class windows close; preserve ownership on failure.
    ///
    /// # Errors
    /// Returns a native failure, including a class that still owns live windows.
    fn close(&mut self) -> Result<(), NativeError> {
        if self.registered {
            // SAFETY: This owner registered this exact class/module pair.
            if unsafe { UnregisterClassW(w!("NevroTick.UI-M0"), self.instance) } == 0 {
                return Err(NativeError::last("UnregisterClassW"));
            }
            self.registered = false;
        }
        Ok(())
    }
}

impl Drop for WindowClass {
    fn drop(&mut self) {
        // Preserve an earlier returned error; normal completion checks close().
        if let Err(error) = self.close() {
            eprintln!("UI-M0 class cleanup: {error}");
        }
    }
}

/// Sole owning top-level HWND; its lifetime also pins callback storage.
struct Window<'state> {
    /// Borrow lasts until destruction, including creation-failure callbacks.
    state: &'state State,
}

impl<'state> Window<'state> {
    /// Create a hidden standard window, borrowing stable state through teardown.
    ///
    /// # Errors
    /// Returns window-creation or callback-state association failure.
    fn create(class: &WindowClass, state: &'state State) -> Result<Self, NativeError> {
        // Keep parent background erasure beneath transparent group-box interiors.
        // WS_CLIPCHILDREN would retain pixels from a previously visible page.
        // SAFETY: Registered class and static strings are valid. The stable state
        // outlives the HWND; WM_NCCREATE stores only this pointer. No menu exists.
        let handle = unsafe {
            CreateWindowExW(
                0,
                w!("NevroTick.UI-M0"),
                w!("NevroTick — Prototype — sample data"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                900,
                680,
                ptr::null_mut(),
                ptr::null_mut(),
                class.instance,
                ptr::from_ref(state).cast::<c_void>(),
            )
        };
        if handle.is_null() {
            return Err(state
                .failure
                .get()
                .unwrap_or_else(|| NativeError::last("CreateWindowExW")));
        }
        Ok(Self { state })
    }

    /// Destroy the original live window and its children, never a recycled HWND.
    ///
    /// # Errors
    /// Returns native destruction failure while retaining the liveness marker.
    fn close(&self) -> Result<(), NativeError> {
        let handle = self.state.window.get();
        if !handle.is_null() {
            // SAFETY: State liveness identifies this thread's original window;
            // WM_NCDESTROY clears it during destruction before handle recycling.
            if unsafe { DestroyWindow(handle) } == 0 {
                return Err(NativeError::last("DestroyWindow"));
            }
        }
        Ok(())
    }
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        if self.close().is_err() {
            // A still-live native callback may retain the State pointer. Never
            // let Rust drop that storage after an unconfirmed destruction. Do
            // not format/write a diagnostic here: stderr failure can unwind.
            std::process::abort();
        }
    }
}

/// Run the fixture-only window on the calling thread until Close.
///
/// All native resources remain owned through normal completion and recoverable
/// errors. An unexpected Rust panic in the callback aborts this transient process
/// instead of unwinding through Windows. No persistent state is created.
///
/// # Errors
/// Returns a checked fixture, native creation, dispatch or cleanup failure.
pub fn run() -> Result<(), NativeError> {
    let model = PreviewModel::fixture()
        .map_err(|_| NativeError::Contract("Checked-in source fixtures are invalid."))?;
    let state = Box::new(State::new(model));
    let mut class = WindowClass::register()?;
    let window = Window::create(&class, &state)?;
    let controls = Controls::create(state.window.get(), class.instance)?;
    state
        .controls
        .set(controls)
        .map_err(|_| NativeError::Contract("Controls were initialized twice."))?;
    state.refresh_sources()?;
    state.show_page(Page::Status)?;
    // SAFETY: Window is owned and live on this UI thread; previous visibility is
    // informational, not a success indicator.
    unsafe { ShowWindow(state.window.get(), SW_SHOW) };
    let result = message_loop().and_then(|()| state.failure.get().map_or(Ok(()), Err));
    let destruction = window.close();
    drop(window);
    let unregister = class.close();
    result.and(destruction).and(unregister)
}

/// Dispatch until `WM_QUIT`; distinguish the documented -1 error from termination.
///
/// # Errors
/// Returns a native message-retrieval failure; callbacks report through State.
fn message_loop() -> Result<(), NativeError> {
    let mut message = MSG::default();
    loop {
        // SAFETY: Writable MSG storage is live; null HWND admits this thread's
        // messages, including WM_QUIT after the original HWND has been destroyed.
        match unsafe { GetMessageW(&raw mut message, ptr::null_mut(), 0, 0) } {
            -1 => return Err(NativeError::last("GetMessageW")),
            0 => return Ok(()),
            _ => {
                // SAFETY: GetMessageW initialized the complete MSG.
                unsafe { TranslateMessage(&raw const message) };
                // SAFETY: MSG came from this thread's queue and remains live.
                unsafe { DispatchMessageW(&raw const message) };
            }
        }
    }
}

/// Reinterpret an exposed pointer address in the ABI's signed pointer-sized slot.
fn pointer_parameter<T>(pointer: *const T) -> LPARAM {
    isize::from_ne_bytes(pointer.expose_provenance().to_ne_bytes())
}

/// Recover pointer provenance for an ABI slot previously supplied by native code.
fn parameter_pointer<T>(value: LPARAM) -> *const T {
    ptr::with_exposed_provenance(usize::from_ne_bytes(value.to_ne_bytes()))
}

/// Callback boundary; Windows owns message validity and this thread's dispatch.
/// Rust panics never cross this ABI. Recoverable errors request owned teardown.
///
/// # Safety
/// Windows must supply this registered class's HWND and correctly marshalled
/// system-message pointers on its owning thread. The original create parameter
/// must be the run-owned State, alive through final destruction.
unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    word: WPARAM,
    long: LPARAM,
) -> LRESULT {
    match std::panic::catch_unwind(|| dispatch(window, message, word, long)) {
        Ok(Ok(Some(result))) => result,
        Ok(Ok(None)) => {
            // SAFETY: Unhandled message and parameters are forwarded unchanged.
            unsafe { DefWindowProcW(window, message, word, long) }
        }
        Ok(Err(error)) => {
            // SAFETY: The callback-scoped borrow ends before returning to
            // Windows; run keeps the original State alive through destruction.
            if let Some(state) = unsafe { callback_state_pointer(window).as_ref() } {
                state.fail(error);
            }
            0
        }
        Err(_) => std::process::abort(),
    }
}

/// Obtain the association without manufacturing an unconstrained Rust lifetime.
/// Only callback-local unsafe blocks may borrow this run-owned storage.
fn callback_state_pointer(window: HWND) -> *const State {
    // SAFETY: Called only during this registered class's synchronous dispatch.
    let value = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) };
    parameter_pointer::<State>(value)
}

/// Attach the create-call's stable state; reject a second creation association.
///
/// # Errors
/// Returns a missing creation record/state. Native association failure is stored
/// in the supplied state and returns the native creation-rejection value zero.
fn attach_state(window: HWND, long: LPARAM) -> Result<LRESULT, NativeError> {
    if !callback_state_pointer(window).is_null() {
        return Ok(0);
    }
    let creation = parameter_pointer::<CREATESTRUCTW>(long);
    // SAFETY: Windows supplies a live CREATESTRUCTW for WM_NCCREATE, including
    // the original lpParam borrowed from Window::create on this same stack.
    let creation = unsafe { creation.as_ref() }
        .ok_or(NativeError::Contract("Missing window creation record."))?;
    // SAFETY: Window::create alone calls this registered class with a live State.
    let state = unsafe { creation.lpCreateParams.cast::<State>().as_ref() }
        .ok_or(NativeError::Contract("Missing window callback state."))?;
    // SAFETY: Clear only this thread's last-error slot before an ambiguous return.
    unsafe { SetLastError(0) };
    // SAFETY: Own window has a pointer-sized user slot; run owns the stable State.
    let previous = unsafe {
        SetWindowLongPtrW(
            window,
            GWLP_USERDATA,
            pointer_parameter(ptr::from_ref(state)),
        )
    };
    let error = NativeError::last("SetWindowLongPtrW");
    if previous == 0 && !matches!(error, NativeError::Win32 { code: 0, .. }) {
        state.failure.set(Some(error));
        return Ok(0);
    }
    state.window.set(window);
    Ok(1)
}

/// Handle only lifecycle, bounded layout and recognized local control actions.
///
/// # Errors
/// Returns invalid native layout/selection, failed model borrowing or a failed
/// control operation, preserving the first failure for ordinary teardown.
fn dispatch(
    window: HWND,
    message: u32,
    word: WPARAM,
    long: LPARAM,
) -> Result<Option<LRESULT>, NativeError> {
    if message == WM_NCCREATE {
        // Successful association does not replace default nonclient creation:
        // DefWindowProcW must still initialize the native caption and state.
        return attach_state(window, long)
            .map(|accepted| if accepted == 0 { Some(0) } else { None });
    }
    if message == WM_GETMINMAXINFO {
        // SAFETY: Windows provides writable MINMAXINFO storage for this system
        // message. Only tracking-size fields change; inherited behavior remains.
        let limits = unsafe { parameter_pointer::<MINMAXINFO>(long).cast_mut().as_mut() }
            .ok_or(NativeError::Contract("Missing native size constraints."))?;
        limits.ptMinTrackSize.x = 760;
        limits.ptMinTrackSize.y = 620;
        return Ok(Some(0));
    }
    // SAFETY: attach_state associates run's stable allocation. This borrow is
    // confined to this callback; no native operation owns or frees that storage.
    let Some(state) = (unsafe { callback_state_pointer(window).as_ref() }) else {
        return Ok(None);
    };
    match message {
        WM_NCDESTROY => {
            // SAFETY: Remove this owner's pointer before Windows recycles HWND.
            unsafe { SetWindowLongPtrW(window, GWLP_USERDATA, 0) };
            state.window.set(ptr::null_mut());
            // SAFETY: Only this thread's single fixture window is closing.
            unsafe { PostQuitMessage(0) };
            Ok(None)
        }
        WM_SIZE => {
            state.layout()?;
            Ok(Some(0))
        }
        WM_COMMAND => {
            state.command(word, long)?;
            Ok(Some(0))
        }
        _ => Ok(None),
    }
}
