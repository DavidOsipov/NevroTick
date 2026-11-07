// SPDX-License-Identifier: GPL-3.0-only
//! Windows-only ordinary-user notification-area observer, not service authority.
//!
//! Source proposal: the direct UI `Win32_UI_Shell` feature and this adapter need
//! graph/source admission and actual Windows qualification before integration.
//! The worker owns a hidden top-level `HWND` so `TaskbarCreated` broadcasts arrive.
//! No raw message pointer is dereferenced; native IDs are scalar checked values.
//! The UI owner remains alive/visible until registration+version acknowledge,
//! and consumes observed loss before acknowledging a restored main window.

use std::cell::RefCell;
use std::ffi::c_void;
use std::fmt;
use std::ptr::{null, null_mut, with_exposed_provenance_mut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateBitmap, CreateDIBSection, DIB_RGB_COLORS,
    DeleteObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION,
    NIN_SELECT, NINF_KEY, NOTIFYICON_VERSION_4, NOTIFYICONDATAW, Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreateIconIndirect, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyIcon,
    DestroyMenu, DestroyWindow, DispatchMessageW, EndMenu, GetCursorPos, GetMessageW, ICONINFO,
    MF_GRAYED, MF_STRING, MSG, PostMessageW, PostQuitMessage, RegisterClassW,
    RegisterWindowMessageW, SetForegroundWindow, TrackPopupMenuEx, TranslateMessage,
    UnregisterClassW, WM_APP, WM_CLOSE, WM_CONTEXTMENU, WM_CREATE, WM_DESTROY, WM_NCCREATE,
    WM_NCDESTROY, WM_NULL, WNDCLASSW, WS_OVERLAPPED,
};

use super::{
    CompletionLatch, ICON_ID, IconCalls, IconOperation, PreparedTray, TrayBridge, TrayError,
    native_command, popup_selection, registration_update,
};

/// Private wake; its raw message alone changes no trusted field.
const CONTROL_WAKE: u32 = WM_APP + 17;
/// Untrusted scalar notification callback, fixed for this one observer.
const ICON_EVENT: u32 = WM_APP + 18;
/// Exact class name; no path, contributor string or external registration is used.
const CLASS: &[u16] = &[
    78, 101, 118, 114, 111, 84, 105, 99, 107, 46, 84, 114, 97, 121, 46, 118, 49, 0,
];
/// Explorer's documented broadcast name, encoded as a fixed NUL-terminated string.
const TASKBAR: &[u16] = &[
    84, 97, 115, 107, 98, 97, 114, 67, 114, 101, 97, 116, 101, 100, 0,
];
/// One native observer per process. The Root owns the only start/stop intention.
static ACTIVE: AtomicBool = AtomicBool::new(false);
thread_local! {
    /// Actual worker-owned association; incoming LPARAM is never a Rust pointer.
    static CURRENT: RefCell<Option<Arc<TrayBridge>>> = const { RefCell::new(None) };
}

/// Fixed startup/cleanup error; no arbitrary Windows message or text is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartError {
    /// A previous observer still owns its native worker.
    AlreadyRunning,
    /// The ordinary worker thread could not be created.
    Worker,
    /// A native class/window/icon operation failed; no availability is assumed.
    Native,
    /// The worker panicked; no panic crosses the Windows callback ABI.
    WorkerPanic,
    /// Completion/cleanup has not actually been confirmed.
    Cleanup,
}
impl fmt::Display for StartError {
    /// Fixed, redacted UI-safe error text.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("The tray observer could not be confirmed. Keep the main window open.")
    }
}
impl std::error::Error for StartError {}

/// Fixed32x32 premultiplied BGRA pixels, prepared from code-owned brand data.
///
/// Preparation belongs on the Slint UI thread using the embedded maintained PNG.
/// This adapter accepts no file path, image decoder or contributor-selected icon.
#[derive(Clone)]
pub struct TrayPixels([u8; 4096]);
impl TrayPixels {
    /// Retain exactly the caller's code-owned32x32 pixel buffer.
    #[must_use]
    pub const fn from_brand_bgra32(pixels: [u8; 4096]) -> Self {
        Self(pixels)
    }

    /// Decode only the compiled maintained brand asset on the Slint UI thread.
    /// Original PNG bytes/notices remain unchanged; no runtime path/import exists.
    ///
    /// # Errors
    /// Reject a changed header/dimension, unavailable public decoder or buffer shape.
    pub fn embedded_brand() -> Result<Self, StartError> {
        const BRAND: &[u8] = include_bytes!("../../ui/assets/head.png");
        if BRAND.len() != 1_116_487
            || BRAND.get(..8) != Some(&[137, 80, 78, 71, 13, 10, 26, 10])
            || BRAND.get(16..24) != Some(&[0, 0, 4, 230, 0, 0, 4, 230])
        {
            return Err(StartError::Native);
        }
        let image =
            slint::Image::load_from_data(BRAND, Some("png")).map_err(|_| StartError::Native)?;
        let pixels = image.to_rgba8_premultiplied().ok_or(StartError::Native)?;
        if pixels.width() != 1254 || pixels.height() != 1254 {
            return Err(StartError::Native);
        }
        let source = pixels.as_bytes();
        if source.len() != 6_290_064 {
            return Err(StartError::Native);
        }
        let mut result = [0_u8; 4096];
        for (y, row) in result.as_chunks_mut::<128>().0.iter_mut().enumerate() {
            let source_y = y.checked_mul(1254).ok_or(StartError::Native)? / 32;
            for (x, pixel) in row.as_chunks_mut::<4>().0.iter_mut().enumerate() {
                let source_x = x.checked_mul(1254).ok_or(StartError::Native)? / 32;
                let offset = source_y
                    .checked_mul(1254)
                    .and_then(|v| v.checked_add(source_x))
                    .and_then(|v| v.checked_mul(4))
                    .ok_or(StartError::Native)?;
                let rgba = source
                    .get(offset..offset.checked_add(4).ok_or(StartError::Native)?)
                    .ok_or(StartError::Native)?;
                pixel.copy_from_slice(&[rgba[2], rgba[1], rgba[0], rgba[3]]);
            }
        }
        Ok(Self(result))
    }
}

/// Sole ordinary native worker owner. Drop is only a best-effort stop request.
///
/// Before acknowledged UI exit the host must call `request_stop` and keep the
/// visible observer/event loop until `try_finish` reports `Ok(true)`. Pending or
/// failed cleanup cannot be treated as permission to hide/quit successfully.
pub struct NativeTray {
    /// Shared bounded projection/event custody, never the Root controller.
    bridge: Arc<TrayBridge>,
    /// Exactly one owned worker, joined only after it reports finished.
    worker: Option<JoinHandle<Result<(), StartError>>>,
    /// First observed terminal outcome remains latched across repeated polls.
    completion: CompletionLatch<StartError>,
}
impl NativeTray {
    /// Lazily start only for the accepted opt-in, retaining Unknown/Unavailable meanwhile.
    ///
    /// # Errors
    /// Reject another active worker or a failed thread creation without native readiness.
    pub fn start(bridge: Arc<TrayBridge>, pixels: TrayPixels) -> Result<Self, StartError> {
        if ACTIVE
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(StartError::AlreadyRunning);
        }
        if bridge.worker_started.swap(true, Ordering::AcqRel) {
            ACTIVE.store(false, Ordering::Release);
            return Err(StartError::AlreadyRunning);
        }
        let owner = Arc::clone(&bridge);
        let result = thread::Builder::new()
            .name("nevrotick-tray".into())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    worker(&owner, &pixels)
                }));
                owner.registered.store(false, Ordering::Release);
                if let Ok(mut endpoint) = owner.endpoint.lock() {
                    *endpoint = 0;
                }
                CURRENT.with(|slot| {
                    if let Ok(mut slot) = slot.try_borrow_mut() {
                        *slot = None;
                    }
                });
                ACTIVE.store(false, Ordering::Release);
                if let Ok(result) = result {
                    if result.is_err() || owner.cleanup_failed.load(Ordering::Acquire) {
                        owner.publish(None, true, true);
                        result.and(Err(StartError::Cleanup))
                    } else {
                        result
                    }
                } else {
                    owner.publish(None, true, true);
                    Err(StartError::WorkerPanic)
                }
            });
        if let Ok(worker) = result {
            Ok(Self {
                bridge,
                worker: Some(worker),
                completion: CompletionLatch::default(),
            })
        } else {
            ACTIVE.store(false, Ordering::Release);
            bridge.worker_started.store(false, Ordering::Release);
            Err(StartError::Worker)
        }
    }
    /// Whether a joined worker positively confirmed every owned native cleanup.
    ///
    /// A startup/runtime native error remains reported by `try_finish`, but is
    /// distinct from cleanup uncertainty. Panic, pending work and any latched
    /// destructive-call failure can never supply this acknowledgement.
    #[must_use]
    pub fn cleanup_confirmed(&self) -> bool {
        matches!(
            self.completion.result(),
            Some(Ok(()) | Err(StartError::Native))
        ) && !self.bridge.cleanup_failed.load(Ordering::Acquire)
    }

    /// Publish a completely prepared candidate and wake only this owned worker.
    ///
    /// # Errors
    /// Reject a poisoned bridge or failed native wake; no update ack is fabricated.
    pub fn update(&self, projection: PreparedTray) -> Result<(), TrayError> {
        if self.bridge.stopping.load(Ordering::Acquire)
            || self.bridge.cleanup_failed.load(Ordering::Acquire)
        {
            return Err(TrayError::BridgeUnavailable);
        }
        self.bridge.replace(projection)?;
        if post_control(&self.bridge).is_err() {
            self.bridge.registered.store(false, Ordering::Release);
            self.bridge.cleanup_failed.store(true, Ordering::Release);
            return Err(TrayError::BridgeUnavailable);
        }
        Ok(())
    }
    /// Request stop from private owner state; a raw `CONTROL_WAKE` is insufficient.
    ///
    /// # Errors
    /// Native posting failure is unconfirmed, not a completed stop.
    pub fn request_stop(&self) -> Result<(), TrayError> {
        self.bridge.stopping.store(true, Ordering::Release);
        post_control(&self.bridge).map_err(|_| TrayError::CleanupUnconfirmed)
    }
    /// Poll completion without an unbounded native wait or joining a running worker.
    ///
    /// # Errors
    /// An acknowledged worker failure, panic or cleanup uncertainty remains an error.
    pub fn try_finish(&mut self) -> Result<bool, StartError> {
        if let Some(result) = self.completion.result() {
            return result.map(|()| true);
        }
        let Some(worker) = self.worker.as_ref() else {
            return self
                .completion
                .finish(Err(StartError::Cleanup))
                .map(|()| true);
        };
        if !worker.is_finished() {
            return Ok(false);
        }
        let Some(worker) = self.worker.take() else {
            return self
                .completion
                .finish(Err(StartError::Cleanup))
                .map(|()| true);
        };
        let result = worker
            .join()
            .map_err(|_| StartError::WorkerPanic)
            .and_then(|value| value)
            .and_then(|()| {
                if self.bridge.cleanup_failed.load(Ordering::Acquire) {
                    Err(StartError::Cleanup)
                } else {
                    Ok(())
                }
            });
        self.completion.finish(result).map(|()| true)
    }
}
impl Drop for NativeTray {
    /// Never claim synchronous completion; the host owns the explicit finish protocol.
    fn drop(&mut self) {
        let _stop = self.request_stop();
    }
}

/// Convert an actual owned opaque handle word without dereferencing Rust memory.
fn hwnd(bridge: &TrayBridge) -> HWND {
    bridge.endpoint.lock().map_or(null_mut(), |value| {
        with_exposed_provenance_mut::<c_void>(*value)
    })
}

/// Queue one private-state wake. No `HWND` exists yet during bounded startup.
///
/// # Errors
/// Return failure only after a real native posting attempt fails.
#[expect(
    unsafe_code,
    reason = "The nonzero HWND came only from this worker's successful CreateWindowExW; no pointer is dereferenced"
)]
fn post_control(bridge: &TrayBridge) -> Result<(), StartError> {
    if bridge.control_queued.swap(true, Ordering::AcqRel) {
        return Ok(());
    }
    let Ok(endpoint) = bridge.endpoint.lock() else {
        bridge.control_queued.store(false, Ordering::Release);
        return Err(StartError::Native);
    };
    let window = with_exposed_provenance_mut::<c_void>(*endpoint);
    if window.is_null() {
        bridge.control_queued.store(false, Ordering::Release);
        return Ok(());
    }
    // SAFETY: retirement takes this same mutex before destruction, preventing
    // HWND reuse during the nonblocking pointer-free PostMessage call. The
    // guard never spans SendMessage, dispatch, menu, create or destroy operations.
    if unsafe { PostMessageW(window, CONTROL_WAKE, 0, 0) } == 0 {
        bridge.control_queued.store(false, Ordering::Release);
        Err(StartError::Native)
    } else {
        Ok(())
    }
}

/// Exactly one worker-thread owner for all long-lived native resources.
struct Resources<'a> {
    /// Shared result sink outlives this worker-local guard.
    bridge: &'a TrayBridge,
    /// Borrowed process module; it must not be `FreeLibrary`'d.
    instance: windows_sys::Win32::Foundation::HINSTANCE,
    /// Registration is ours only after `RegisterClassW` actually succeeds.
    class_registered: bool,
    /// Unique native top-level window, never a foreign `HWND`.
    window: HWND,
    /// Unique created icon, released after shell deletion/window destruction.
    icon: IconHandle,
    /// Explicit cleanup has already run; never retry uncertain destructive calls.
    finished: bool,
}
impl Resources<'_> {
    /// Release each actually acquired resource on every return/panic path.
    ///
    /// # Errors
    /// Any failed native cleanup is latched, never reported as completed.
    #[expect(
        unsafe_code,
        reason = "Each block releases one matching resource owned by this exact worker; handles are taken before calls"
    )]
    fn finish(&mut self) -> Result<(), StartError> {
        if self.finished {
            return Ok(());
        }
        self.finished = true;
        self.bridge.stopping.store(true, Ordering::Release);
        self.bridge.registered.store(false, Ordering::Release);
        let deleted = !self.bridge.icon_attempted.swap(false, Ordering::AcqRel)
            || notify(self.bridge, NIM_DELETE);
        let window = std::mem::replace(&mut self.window, null_mut());
        match self.bridge.endpoint.lock() {
            Ok(mut value) => *value = 0,
            Err(poison) => {
                *poison.into_inner() = 0;
                self.bridge.cleanup_failed.store(true, Ordering::Release);
            }
        }
        let window_removed = window.is_null() || {
            // SAFETY: this is our own worker HWND; all synchronous popup loops have returned.
            (unsafe { DestroyWindow(window) }) != 0
        };

        let icon = std::mem::replace(&mut self.icon, null_mut());
        let icon_removed = icon.is_null() || {
            // SAFETY: our unique created HICON has not escaped ownership and shell removal was attempted.
            (unsafe { DestroyIcon(icon) }) != 0
        };
        self.bridge.icon.store(0, Ordering::Release);
        let class_removed = !self.class_registered || {
            self.class_registered = false;
            // SAFETY: our own class is released only after the owned window destruction attempt.
            (unsafe { UnregisterClassW(CLASS.as_ptr(), self.instance) }) != 0
        };
        if deleted && window_removed && icon_removed && class_removed {
            Ok(())
        } else {
            self.bridge.cleanup_failed.store(true, Ordering::Release);
            Err(StartError::Cleanup)
        }
    }
}
impl Drop for Resources<'_> {
    /// Startup failures and unwind paths retain the same explicit cleanup owner.
    fn drop(&mut self) {
        let _cleanup = self.finish();
    }
}

/// Establish resources on their owning native thread, then service its message queue.
///
/// # Errors
/// Every failed create/registration/loop/cleanup remains explicit and unavailable.
#[expect(
    unsafe_code,
    reason = "Each native operation uses initialized owned buffers/handles on this worker; individual safety contracts appear at each block"
)]
fn worker(bridge: &Arc<TrayBridge>, pixels: &TrayPixels) -> Result<(), StartError> {
    CURRENT.with(|slot| {
        *slot.borrow_mut() = Some(Arc::clone(bridge));
    });
    // SAFETY: null names the current loaded process module; returned handle is borrowed.
    let instance = unsafe { GetModuleHandleW(null()) };
    if instance.is_null() {
        return Err(StartError::Native);
    }
    let mut resources = Resources {
        bridge,
        instance,
        class_registered: false,
        window: null_mut(),
        icon: null_mut(),
        finished: false,
    };
    let class = WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null(),
        lpszClassName: CLASS.as_ptr(),
    };
    // SAFETY: initialized class, static terminated name and callback live for this entire worker.
    let atom = unsafe { RegisterClassW(&raw const class) };
    if atom == 0 {
        return Err(StartError::Native);
    }
    resources.class_registered = true;
    // SAFETY: registered class and module remain live; no raw userdata pointer is supplied.
    resources.window = unsafe {
        CreateWindowExW(
            0,
            CLASS.as_ptr(),
            CLASS.as_ptr(),
            WS_OVERLAPPED,
            0,
            0,
            0,
            0,
            null_mut(),
            null_mut(),
            instance,
            null(),
        )
    };
    if resources.window.is_null() {
        return Err(StartError::Native);
    }
    *bridge.endpoint.lock().map_err(|_| StartError::Native)? = resources.window.expose_provenance();
    // SAFETY: fixed initialized terminated broadcast name, not contributor input.
    let taskbar = unsafe { RegisterWindowMessageW(TASKBAR.as_ptr()) };
    if taskbar == 0 {
        return Err(StartError::Native);
    }
    bridge.taskbar_created.store(
        usize::try_from(taskbar).map_err(|_| StartError::Native)?,
        Ordering::Release,
    );
    resources.icon = create_icon(pixels)?;
    bridge
        .icon
        .store(resources.icon.expose_provenance(), Ordering::Release);
    let mut run_result = if bridge.stopping.load(Ordering::Acquire) {
        Ok(())
    } else {
        publish_icon(bridge, false)
    };
    if run_result.is_ok() && !bridge.stopping.load(Ordering::Acquire) {
        let mut message = MSG::default();
        loop {
            // SAFETY: initialized writable record on this owning message-loop thread.
            let result = unsafe { GetMessageW(&raw mut message, null_mut(), 0, 0) };
            if result == -1 {
                run_result = Err(StartError::Native);
                break;
            }
            if result == 0 {
                if !bridge.stopping.load(Ordering::Acquire) {
                    run_result = Err(StartError::Native);
                }
                break;
            }
            // SAFETY: this is the current OS-produced initialized message record.
            let _translated = unsafe { TranslateMessage(&raw const message) };
            // SAFETY: the same OS record remains live throughout synchronous dispatch.
            let _dispatched = unsafe { DispatchMessageW(&raw const message) };
        }
    }
    let cleanup = resources.finish();
    cleanup.and(run_result)
}

/// Build one copied alpha icon, releasing both temporary bitmaps exactly once.
///
/// # Errors
/// Reject any failed bitmap/icon construction or bitmap cleanup.
#[expect(
    unsafe_code,
    reason = "Only fixed32x32 initialized buffers are passed; HBITMAP ownership is retained through copying and released once"
)]
fn create_icon(pixels: &TrayPixels) -> Result<IconHandle, StartError> {
    let header_size =
        u32::try_from(size_of::<BITMAPINFOHEADER>()).map_err(|_| StartError::Native)?;
    let mask = [0_u8; 128];
    // SAFETY: fixed128-byte1bpp32x32 mask has the API-required word-aligned stride.
    let mask_bitmap = unsafe { CreateBitmap(32, 32, 1, 1, mask.as_ptr().cast()) };
    if mask_bitmap.is_null() {
        return Err(StartError::Native);
    }
    let bitmap = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: header_size,
            biWidth: 32,
            biHeight: -32,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 4096,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        ..BITMAPINFO::default()
    };
    let mut destination = null_mut();
    // SAFETY: initialized32x32top-down32bpp header and writable returned-pointer slot; no section handle is borrowed.
    let color_bitmap = unsafe {
        CreateDIBSection(
            null_mut(),
            &raw const bitmap,
            DIB_RGB_COLORS,
            &raw mut destination,
            null_mut(),
            0,
        )
    };
    if color_bitmap.is_null() || destination.is_null() {
        // SAFETY: unique successful mask bitmap, not used by any icon.
        let mask_released = unsafe { DeleteObject(mask_bitmap) } != 0;
        let mut color_released = true;
        if !color_bitmap.is_null() {
            // SAFETY: a returned bitmap is still ours even if the returned bits pointer was unusable.
            color_released = unsafe { DeleteObject(color_bitmap) } != 0;
        }
        return Err(if mask_released && color_released {
            StartError::Native
        } else {
            StartError::Cleanup
        });
    }
    // SAFETY: successful fixed4096-byte DIB allocation is writable and disjoint from our fixed owned source array.
    unsafe { std::ptr::copy_nonoverlapping(pixels.0.as_ptr(), destination.cast::<u8>(), 4096) };
    let info = ICONINFO {
        fIcon: 1,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: mask_bitmap,
        hbmColor: color_bitmap,
    };
    // SAFETY: both initialized owned bitmaps stay valid while CreateIconIndirect copies them.
    let icon = unsafe { CreateIconIndirect(&raw const info) };
    // SAFETY: CreateIconIndirect copies both bitmaps; this worker still uniquely owns each temporary bitmap.
    let mask_removed = unsafe { DeleteObject(mask_bitmap) } != 0;
    // SAFETY: the second unique temporary bitmap is no longer needed after the native copy.
    let color_removed = unsafe { DeleteObject(color_bitmap) } != 0;
    if icon.is_null() {
        return Err(if mask_removed && color_removed {
            StartError::Native
        } else {
            StartError::Cleanup
        });
    }
    if !mask_removed || !color_removed {
        // SAFETY: created icon ownership has not escaped this function.
        let _removed = unsafe { DestroyIcon(icon) };
        return Err(StartError::Cleanup);
    }
    Ok(icon)
}
/// Opaque `HICON` alias, confined to this native module.
type IconHandle = windows_sys::Win32::UI::WindowsAndMessaging::HICON;

/// Send an initialized complete notification record; message payload never chooses it.
#[expect(
    unsafe_code,
    reason = "NOTIFYICONDATAW is fully initialized and all borrowed native handles remain owned by the worker"
)]
fn notify(bridge: &TrayBridge, operation: u32) -> bool {
    let projection = if operation == NIM_ADD || operation == NIM_MODIFY {
        match bridge.prepared_projection() {
            Ok(value) => Some(value),
            Err(_) => return false,
        }
    } else {
        None
    };
    let Ok(size) = u32::try_from(size_of::<NOTIFYICONDATAW>()) else {
        return false;
    };
    let mut record = NOTIFYICONDATAW {
        cbSize: size,
        hWnd: hwnd(bridge),
        uID: u32::from(ICON_ID),
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP | NIF_SHOWTIP,
        uCallbackMessage: ICON_EVENT,
        hIcon: with_exposed_provenance_mut(bridge.icon.load(Ordering::Acquire)),
        ..NOTIFYICONDATAW::default()
    };
    if let Some(projection) = projection {
        record.szTip = *projection.tooltip().units();
    }
    if operation == NIM_ADD {
        bridge.icon_attempted.store(true, Ordering::Release);
    }
    record.Anonymous.uVersion = NOTIFYICON_VERSION_4;
    // SAFETY: the complete fixed-size record and all literal terminated text stay live during the synchronous call.
    (unsafe { Shell_NotifyIconW(operation, &raw const record) }) != 0
}

/// Registration/version acknowledge only their actual native API outcomes.
///
/// # Errors
/// Any failed acknowledgement invalidates readiness and requests visible UI restoration.
fn publish_icon(bridge: &TrayBridge, registered: bool) -> Result<(), StartError> {
    struct PublishGuard<'a>(&'a TrayBridge);
    impl Drop for PublishGuard<'_> {
        fn drop(&mut self) {
            self.0.publishing.store(false, Ordering::Release);
        }
    }
    struct Calls<'a>(&'a TrayBridge);
    impl IconCalls for Calls<'_> {
        fn notify(&mut self, operation: IconOperation) -> bool {
            let operation = match operation {
                IconOperation::Add => NIM_ADD,
                IconOperation::SetVersion => NIM_SETVERSION,
                IconOperation::Modify => NIM_MODIFY,
            };
            notify(self.0, operation)
        }
    }
    if bridge.publishing.swap(true, Ordering::AcqRel) {
        return Err(StartError::Native);
    }
    let publishing = PublishGuard(bridge);
    if hwnd(bridge).is_null()
        || bridge.icon.load(Ordering::Acquire) == 0
        || bridge.taskbar_created.load(Ordering::Acquire) == 0
        || bridge.stopping.load(Ordering::Acquire)
    {
        bridge.registered.store(false, Ordering::Release);
        return Err(StartError::Native);
    }
    // Consume only after the publication barrier is held, before reading the
    // snapshot. Later host updates leave pending=true until their own ack.
    bridge.update_pending.store(false, Ordering::Release);
    let loss_before = bridge.loss_generation.load(Ordering::Acquire);
    let success = registration_update(&mut Calls(bridge), registered)
        && bridge.loss_generation.load(Ordering::Acquire) == loss_before;
    bridge.registered.store(success, Ordering::Release);
    drop(publishing);
    bridge.publish(None, !success, !success);
    if success {
        Ok(())
    } else {
        Err(StartError::Native)
    }
}

/// FFI boundary: no Rust panic or raw message pointer can escape into Windows.
///
/// # Safety
/// Windows must invoke this registered callback with the actual window and
/// message ABI on its owning worker thread. Forwarded system messages retain
/// their documented Win32 payload contract; no LPARAM is read as Rust storage.
#[expect(
    unsafe_code,
    reason = "The system callback ABI is unsafe; scalar validation precedes all effects and no LPARAM memory is dereferenced"
)]
unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        dispatch(window, message, wparam, lparam)
    }));
    if let Ok(result) = result {
        result
    } else {
        CURRENT.with(|slot| {
            if let Ok(slot) = slot.try_borrow()
                && let Some(bridge) = slot.as_ref()
            {
                bridge.registered.store(false, Ordering::Release);
                bridge.cleanup_failed.store(true, Ordering::Release);
                bridge.record_loss();
            }
        });
        // SAFETY: this only requests exit of the current native worker queue; no pointer or service operation exists.
        unsafe { PostQuitMessage(0) };
        0
    }
}

/// Snapshot TLS association before any potentially reentrant native API call.
#[expect(
    unsafe_code,
    reason = "Unknown messages are forwarded with their original OS scalar parameters; this adapter never dereferences their payload"
)]
fn dispatch(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if message == WM_NCDESTROY {
        // SAFETY: required default nonclient cleanup is never swallowed by our stop state.
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    }
    if message == WM_NCCREATE {
        return 1;
    }
    if message == WM_CREATE {
        return 0;
    }
    let bridge = CURRENT.with(|slot| slot.try_borrow().ok().and_then(|slot| slot.clone()));
    let Some(bridge) = bridge else {
        // SAFETY: ordinary default handling of the callback's current OS window/message.
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    };
    if hwnd(&bridge) != window {
        // SAFETY: early creation messages have no association; no userdata pointer is read.
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    }
    let taskbar = bridge.taskbar_created.load(Ordering::Acquire);
    let is_taskbar = taskbar != 0 && usize::try_from(message).ok() == Some(taskbar);
    if bridge.stopping.load(Ordering::Acquire)
        && (message == CONTROL_WAKE || message == ICON_EVENT || is_taskbar)
    {
        control(&bridge);
        return 0;
    }
    if message == WM_CLOSE || message == WM_DESTROY {
        // These are not permission to quit the app. Fail this observer closed,
        // restore the ordinary Root and end only this failed native worker.
        if !bridge.cleanup_failed.swap(true, Ordering::AcqRel) {
            bridge.publish(None, true, true);
        }
        end_worker(&bridge);
        return 0;
    }
    if message == CONTROL_WAKE {
        bridge.control_queued.store(false, Ordering::Release);
        control(&bridge);
        return 0;
    }
    if is_taskbar {
        bridge.publish(None, false, true);
        let _registered = publish_icon(&bridge, false);
        return 0;
    }
    if message == ICON_EVENT {
        icon_event(&bridge, wparam, lparam);
        return 0;
    }
    // Unsolicited WM_COMMAND cannot manufacture an action or Root exit.
    // SAFETY: unchanged default handling; no external payload is interpreted as Rust memory.
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// End only this worker queue; popup handles remain owned until its loop returns.
#[expect(
    unsafe_code,
    reason = "Only the current thread's own active popup and message queue are ended; no pointer or service effect exists"
)]
fn end_worker(bridge: &TrayBridge) {
    if bridge.menu_open.load(Ordering::Acquire) {
        // SAFETY: this thread owns the currently active synchronous popup.
        let _ended = unsafe { EndMenu() };
    }
    // SAFETY: only the current native worker queue is requested to stop.
    unsafe { PostQuitMessage(0) };
}

/// Close a menu of the previous catalog generation without stopping the worker.
#[expect(
    unsafe_code,
    reason = "Only this worker's currently active popup is ended; resource destruction waits for TrackPopupMenuEx to return"
)]
fn end_menu_for_update() {
    // SAFETY: called only after the worker's private menu_open flag is observed.
    let _ended = unsafe { EndMenu() };
}

/// A raw wake only consumes private host stop/update state, never message arguments.
fn control(bridge: &TrayBridge) {
    if bridge.stopping.load(Ordering::Acquire) {
        end_worker(bridge);
    } else if bridge.update_pending.load(Ordering::Acquire)
        && bridge.menu_open.load(Ordering::Acquire)
    {
        // Close the old-generation menu before changing its catalog projection.
        end_menu_for_update();
    } else if bridge.update_pending.load(Ordering::Acquire) {
        if bridge.icon.load(Ordering::Acquire) == 0 {
            bridge.update_pending.store(true, Ordering::Release);
            return;
        }
        let _updated = publish_icon(bridge, bridge.registered.load(Ordering::Acquire));
    }
}

/// Validate v4 scalar packing and ignore every unsupported/foreign-icon event.
fn icon_event(bridge: &TrayBridge, wparam: WPARAM, lparam: LPARAM) {
    if !bridge.registered.load(Ordering::Acquire) || bridge.stopping.load(Ordering::Acquire) {
        return;
    }
    let data = lparam.to_le_bytes();
    let event = u32::from(u16::from_le_bytes([data[0], data[1]]));
    let icon_id = u16::from_le_bytes([data[2], data[3]]);
    if icon_id != ICON_ID {
        return;
    }
    if event == NIN_SELECT || event == (NIN_SELECT | NINF_KEY) {
        bridge.publish(Some(crate::tray_ui::TrayItemId::Open), false, false);
    } else if event == WM_CONTEXTMENU
        && bridge.availability() == crate::shell::AdapterAvailability::Ready
    {
        let _menu_result = popup(bridge, wparam);
    }
}

/// Own popup resources until `TrackPopupMenuEx` returns, including stop/reentrancy.
///
/// # Errors
/// Native menu failure submits no action. Unconfirmed cleanup prevents another popup.
fn popup(bridge: &TrayBridge, wparam: WPARAM) -> Result<(), StartError> {
    if bridge.cleanup_failed.load(Ordering::Acquire) {
        return Err(StartError::Cleanup);
    }
    if bridge.menu_open.swap(true, Ordering::AcqRel) {
        return Ok(());
    }
    let result = popup_inner(bridge, wparam);
    bridge.menu_open.store(false, Ordering::Release);
    control(bridge);
    if result.is_err() {
        bridge.publish(None, true, false);
    }
    result
}

/// Build the six bounded rows and accept only the native return command.
///
/// # Errors
/// Any native creation/append/cleanup/selection error remains explicit.
#[expect(
    unsafe_code,
    reason = "This function owns exactly one HMENU, six terminated buffers and a borrowed live HWND; each native block performs one operation"
)]
fn popup_inner(bridge: &TrayBridge, wparam: WPARAM) -> Result<(), StartError> {
    let projection = bridge
        .prepared_projection()
        .map_err(|_| StartError::Native)?;
    // SAFETY: CreatePopupMenu has no pointer arguments and returns a uniquely owned menu.
    let menu = unsafe { CreatePopupMenu() };
    if menu.is_null() {
        return Err(StartError::Native);
    }
    let mut build_ok = true;
    for item in projection.items() {
        let id = native_command(item.id());
        let flags = MF_STRING | if item.enabled() { 0 } else { MF_GRAYED };
        // SAFETY: menu is owned, text is terminated/escaped bounded UTF-16, ID is one of1..6.
        if unsafe { AppendMenuW(menu, flags, id, item.text().units().as_ptr()) } == 0 {
            build_ok = false;
            break;
        }
    }
    let data = wparam.to_le_bytes();
    let mut point = windows_sys::Win32::Foundation::POINT {
        x: i32::from(i16::from_le_bytes([data[0], data[1]])),
        y: i32::from(i16::from_le_bytes([data[2], data[3]])),
    };
    if point.x == -1 && point.y == -1 {
        // SAFETY: initialized writable POINT for the current cursor query.
        if unsafe { GetCursorPos(&raw mut point) } == 0 {
            build_ok = false;
        }
    }
    // SAFETY: only this worker's own top-level owner is activated for its popup; failure submits no action.
    let foreground = unsafe { SetForegroundWindow(hwnd(bridge)) } != 0;
    let selected = if build_ok
        && foreground
        && !bridge.stopping.load(Ordering::Acquire)
        && !bridge.update_pending.load(Ordering::Acquire)
    {
        // SAFETY: all rows/owner remain live; TPM_NONOTIFY forbids duplicate WM_COMMAND delivery authority.
        unsafe {
            TrackPopupMenuEx(
                menu,
                windows_sys::Win32::UI::WindowsAndMessaging::TPM_RETURNCMD
                    | windows_sys::Win32::UI::WindowsAndMessaging::TPM_NONOTIFY,
                point.x,
                point.y,
                hwnd(bridge),
                null(),
            )
        }
    } else {
        0
    };
    // SAFETY: synchronous popup returned, so this worker can release its unique menu exactly once.
    let removed = unsafe { DestroyMenu(menu) } != 0;
    let cleanup = bridge.acknowledge_popup_cleanup(removed);
    // SAFETY: this fixed wake has no pointer or action payload; it supports normal foreground-menu dismissal.
    let _dismiss = unsafe { PostMessageW(hwnd(bridge), WM_NULL, 0, 0) };
    cleanup.map_err(|_| StartError::Cleanup)?;
    if !build_ok || !foreground {
        return Err(StartError::Native);
    }
    if bridge.stopping.load(Ordering::Acquire) || bridge.update_pending.load(Ordering::Acquire) {
        return Ok(());
    }
    let action = popup_selection(selected).map_err(|_| StartError::Native)?;
    if let Some(action) = action {
        // Old menu-enabled state is never authority: the actual Root will recheck it.
        bridge.publish(Some(action), false, false);
    }
    Ok(())
}
