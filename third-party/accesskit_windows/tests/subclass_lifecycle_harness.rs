// SPDX-License-Identifier: GPL-3.0-only
//! Controls for the actual subclass module through an inert Win32 facade.
//! This does not exercise USER32, UIPI or Windows scheduling natively.
extern crate self as accesskit;
extern crate self as windows;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};
#[path = "../src/subclass.rs"]
mod subclass;
pub struct TreeUpdate;
pub trait ActionHandler {}
pub trait ActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate>;
}
pub struct QueuedEvents;
impl QueuedEvents {
    pub fn raise(self) {}
}
pub struct Adapter {
    window: usize,
}
impl Adapter {
    pub fn new(
        window: Win32::Foundation::HWND,
        _: bool,
        _: impl 'static + ActionHandler + Send,
    ) -> Self {
        Self {
            window: window.0 as usize,
        }
    }
    pub fn update_if_active(
        &mut self,
        update: impl FnOnce() -> TreeUpdate,
    ) -> Option<QueuedEvents> {
        let _ = update();
        None
    }
    pub fn handle_wm_getobject(
        &mut self,
        _: Win32::Foundation::WPARAM,
        _: Win32::Foundation::LPARAM,
        handler: &mut dyn ActivationHandler,
    ) -> Option<isize> {
        handler.request_initial_tree().map(|_| 19)
    }
    pub fn update_window_focus_state(&mut self, _: bool) -> Option<QueuedEvents> {
        FOCUS.with(|focus| *focus.borrow_mut().entry(self.window).or_default() += 1);
        None
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        DROPS.with(|drops| *drops.borrow_mut().entry(self.window).or_default() += 1);
    }
}
pub mod core {
    pub struct Error;
    impl Error {
        pub fn from_thread() -> Self {
            Self
        }
    }
    impl std::fmt::Display for Error {
        fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            output.write_str("mock Win32 error")
        }
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            output.write_str("mock Win32 error")
        }
    }
    #[derive(Clone, Copy)]
    pub struct PCWSTR(pub usize);
    pub use crate::w;
}
#[macro_export]
macro_rules! w {
    ($value:literal) => {
        $crate::core::PCWSTR(1)
    };
}
#[allow(
    non_snake_case,
    reason = "Match windows-rs API spelling at the inert facade."
)]
pub mod Win32 {
    pub mod Foundation {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct HWND(pub *mut std::ffi::c_void);
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct HANDLE(pub *mut std::ffi::c_void);
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct WPARAM(pub usize);
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct LPARAM(pub isize);
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct LRESULT(pub isize);
        impl From<isize> for LRESULT {
            fn from(value: isize) -> Self {
                Self(value)
            }
        }
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct BOOL(pub i32);
        impl BOOL {
            pub fn as_bool(self) -> bool {
                self.0 != 0
            }
        }
        impl From<BOOL> for bool {
            fn from(value: BOOL) -> Self {
                value.as_bool()
            }
        }
    }
    pub mod System {
        pub mod Threading {
            pub unsafe fn GetCurrentThreadId() -> u32 {
                1
            }
        }
    }
    pub mod UI {
        pub mod WindowsAndMessaging {
            use crate::{
                WINDOWS,
                Win32::Foundation::*,
                core::{Error, PCWSTR},
            };
            pub type WNDPROC =
                Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>;
            pub const GWLP_WNDPROC: i32 = -4;
            pub const WM_GETOBJECT: u32 = 0x003d;
            pub const WM_SETFOCUS: u32 = 7;
            pub const WM_KILLFOCUS: u32 = 8;
            pub const WM_ENTERMENULOOP: u32 = 0x0211;
            pub const WM_EXITMENULOOP: u32 = 0x0212;
            pub const WM_ENTERSIZEMOVE: u32 = 0x0231;
            pub const WM_EXITSIZEMOVE: u32 = 0x0232;
            pub const WM_NCDESTROY: u32 = 0x0082;
            pub unsafe fn GetPropW(window: HWND, _: PCWSTR) -> HANDLE {
                HANDLE(WINDOWS.with(|map| {
                    map.borrow()
                        .get(&(window.0 as usize))
                        .map_or(0, |window| window.property)
                }) as _)
            }
            pub unsafe fn SetPropW(
                window: HWND,
                _: PCWSTR,
                value: Option<HANDLE>,
            ) -> Result<(), Error> {
                WINDOWS.with(|map| {
                    let mut map = map.borrow_mut();
                    let window = map.get_mut(&(window.0 as usize)).ok_or(Error)?;
                    window.property = value.map_or(0, |value| value.0 as usize);
                    Ok(())
                })
            }
            pub unsafe fn RemovePropW(window: HWND, _: PCWSTR) -> Result<HANDLE, Error> {
                WINDOWS.with(|map| {
                    let mut map = map.borrow_mut();
                    let window = map.get_mut(&(window.0 as usize)).ok_or(Error)?;
                    Ok(HANDLE(std::mem::replace(&mut window.property, 0) as _))
                })
            }
            pub unsafe fn SetWindowLongPtrW(window: HWND, _: i32, procedure: isize) -> isize {
                let refuse = crate::FAIL_NEXT_INSTALL.with(|failure| {
                    let mut failure = failure.borrow_mut();
                    if *failure == Some(window.0 as usize) {
                        failure.take();
                        true
                    } else {
                        false
                    }
                });
                if refuse {
                    return 0;
                }
                WINDOWS.with(|map| {
                    map.borrow_mut()
                        .get_mut(&(window.0 as usize))
                        .filter(|window| window.alive)
                        .map_or(0, |window| {
                            std::mem::replace(&mut window.procedure, procedure)
                        })
                })
            }
            pub unsafe fn GetWindowLongPtrW(window: HWND, _: i32) -> isize {
                WINDOWS.with(|map| {
                    map.borrow()
                        .get(&(window.0 as usize))
                        .filter(|window| window.alive)
                        .map_or(0, |window| window.procedure)
                })
            }
            pub unsafe fn IsWindow(window: Option<HWND>) -> BOOL {
                BOOL(i32::from(window.is_some_and(|window| {
                    WINDOWS.with(|map| {
                        map.borrow()
                            .get(&(window.0 as usize))
                            .is_some_and(|window| window.alive)
                    })
                })))
            }
            pub unsafe fn IsWindowVisible(_: HWND) -> BOOL {
                BOOL(0)
            }
            pub unsafe fn GetWindowThreadProcessId(window: HWND, _: Option<*mut u32>) -> u32 {
                WINDOWS.with(|map| {
                    map.borrow()
                        .get(&(window.0 as usize))
                        .filter(|window| window.alive)
                        .map_or(0, |window| window.thread)
                })
            }
            pub unsafe fn CallWindowProcW(
                procedure: WNDPROC,
                window: HWND,
                message: u32,
                wparam: WPARAM,
                lparam: LPARAM,
            ) -> LRESULT {
                // SAFETY: Only source and harness code pointers enter the mock installer.
                procedure.map_or(LRESULT(0), |procedure| unsafe {
                    procedure(window, message, wparam, lparam)
                })
            }
            pub unsafe fn DefWindowProcW(_: HWND, _: u32, _: WPARAM, _: LPARAM) -> LRESULT {
                LRESULT(71)
            }
        }
    }
}
use Win32::{Foundation::*, UI::WindowsAndMessaging::*};
struct Window {
    procedure: isize,
    property: usize,
    alive: bool,
    thread: u32,
}
thread_local! {
    static WINDOWS: RefCell<BTreeMap<usize, Window>> = const { RefCell::new(BTreeMap::new()) };
    static FOCUS: RefCell<BTreeMap<usize, usize>> = const { RefCell::new(BTreeMap::new()) };
    static DROPS: RefCell<BTreeMap<usize, usize>> = const { RefCell::new(BTreeMap::new()) };
    static LATER_PREVIOUS: RefCell<Option<isize>> = const { RefCell::new(None) };
    static FAIL_NEXT_INSTALL: RefCell<Option<usize>> = const { RefCell::new(None) };
}
extern "system" fn original(_: HWND, _: u32, _: WPARAM, _: LPARAM) -> LRESULT {
    LRESULT(71)
}
extern "system" fn later(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let pointer = LATER_PREVIOUS.with(|value| value.borrow().expect("later chain predecessor"));
    // SAFETY: The mock installer returned this exact valid procedure pointer.
    unsafe {
        CallWindowProcW(
            std::mem::transmute::<isize, WNDPROC>(pointer),
            window,
            message,
            wparam,
            lparam,
        )
    }
}
fn create(id: usize) -> HWND {
    let window = HWND(id as _);
    WINDOWS.with(|map| {
        map.borrow_mut().insert(
            id,
            Window {
                procedure: original as *const () as isize,
                property: 0,
                alive: true,
                thread: 1,
            },
        );
    });
    window
}
fn dispatch(window: HWND, message: u32) -> LRESULT {
    // SAFETY: The mock stores only currently installed source/harness procedures.
    unsafe {
        let pointer = GetWindowLongPtrW(window, GWLP_WNDPROC);
        assert_ne!(pointer, 0);
        CallWindowProcW(
            std::mem::transmute::<isize, WNDPROC>(pointer),
            window,
            message,
            WPARAM(0),
            LPARAM(0),
        )
    }
}
struct NoActivation;
impl ActivationHandler for NoActivation {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        None
    }
}
struct NoAction;
impl ActionHandler for NoAction {}
fn adapter(window: HWND) -> subclass::SubclassingAdapter {
    subclass::SubclassingAdapter::new(window, NoActivation, NoAction)
}
fn focuses(window: HWND) -> usize {
    FOCUS.with(|map| map.borrow().get(&(window.0 as usize)).copied().unwrap_or(0))
}
fn drops(window: HWND) -> usize {
    DROPS.with(|map| map.borrow().get(&(window.0 as usize)).copied().unwrap_or(0))
}
fn original_installed(window: HWND) -> bool {
    // SAFETY: Read-only observation of this test's owned mock HWND.
    unsafe { GetWindowLongPtrW(window, GWLP_WNDPROC) == original as *const () as isize }
}
#[test]
fn healthy_focus_and_drop_restore_previous_proc() {
    let window = create(1);
    let mut owner = adapter(window);
    assert!(owner.update_if_active(|| TreeUpdate).is_none());
    assert_eq!(dispatch(window, WM_SETFOCUS), LRESULT(71));
    assert_eq!(focuses(window), 1);
    drop(owner);
    assert!(original_installed(window));
    assert_eq!(drops(window), 1);
}
#[test]
fn peer_property_substitution_cannot_select_another_live_owner() {
    let first = create(2);
    let second = create(3);
    let first_owner = adapter(first);
    let second_owner = adapter(second);
    // SAFETY: Original source publishes two live allocations. The repaired source
    // publishes none and must never read the arbitrary inert replacement value.
    unsafe {
        let second_property = GetPropW(second, w!("AccessKitAdapter"));
        let substitute = if second_property.0.is_null() {
            HANDLE(0x1234usize as _)
        } else {
            second_property
        };
        SetPropW(first, w!("AccessKitAdapter"), Some(substitute)).unwrap();
    }
    dispatch(first, WM_SETFOCUS);
    assert_eq!(
        focuses(first),
        1,
        "callback must use the first window's process-owned state"
    );
    assert_eq!(
        focuses(second),
        0,
        "peer property must not select another owner"
    );
    drop(first_owner);
    drop(second_owner);
}
#[test]
fn forged_destroy_does_not_leave_callback_installed_after_owner_drop() {
    let window = create(4);
    let owner = adapter(window);
    dispatch(window, WM_NCDESTROY);
    drop(owner);
    assert!(
        original_installed(window),
        "a numeric message is not native destruction"
    );
    assert_eq!(dispatch(window, WM_SETFOCUS), LRESULT(71));
    assert_eq!(drops(window), 1);
}
#[test]
fn detached_adapter_does_not_invoke_update_factory() {
    let window = create(12);
    let mut owner = adapter(window);
    let mut attached_called = false;
    assert!(
        owner
            .update_if_active(|| {
                attached_called = true;
                TreeUpdate
            })
            .is_none()
    );
    assert!(attached_called);
    dispatch(window, WM_NCDESTROY);
    let mut called = false;
    assert!(
        owner
            .update_if_active(|| {
                called = true;
                TreeUpdate
            })
            .is_none()
    );
    assert!(!called);
    drop(owner);
}
#[test]
fn failed_install_rolls_back_before_a_healthy_replacement() {
    let window = create(13);
    FAIL_NEXT_INSTALL.with(|failure| *failure.borrow_mut() = Some(13));
    assert!(std::panic::catch_unwind(|| adapter(window)).is_err());
    assert!(original_installed(window));
    assert_eq!(drops(window), 1);
    let replacement = adapter(window);
    dispatch(window, WM_SETFOCUS);
    assert_eq!(focuses(window), 1);
    drop(replacement);
    assert!(original_installed(window));
}
#[test]
fn failed_drop_restore_retains_only_safe_forwarding() {
    let window = create(14);
    let owner = adapter(window);
    FAIL_NEXT_INSTALL.with(|failure| *failure.borrow_mut() = Some(14));
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(owner))).is_err());
    assert_eq!(drops(window), 1);
    assert_eq!(dispatch(window, WM_SETFOCUS), LRESULT(71));
    assert_eq!(focuses(window), 0);
}
#[test]
fn later_subclass_survives_owner_drop_and_forwards_without_live_state() {
    let window = create(5);
    let owner = adapter(window);
    // SAFETY: Install one valid harness-owned procedure above the source callback.
    let previous = unsafe { SetWindowLongPtrW(window, GWLP_WNDPROC, later as *const () as isize) };
    LATER_PREVIOUS.with(|value| *value.borrow_mut() = Some(previous));
    drop(owner);
    // SAFETY: Read only this mock window's current procedure.
    assert_eq!(
        unsafe { GetWindowLongPtrW(window, GWLP_WNDPROC) },
        later as *const () as isize
    );
    assert_eq!(drops(window), 1);
    assert_eq!(dispatch(window, WM_SETFOCUS), LRESULT(71));
    assert_eq!(focuses(window), 0);
}
#[test]
fn duplicate_registration_is_rejected_without_replacing_first() {
    let window = create(6);
    let first = adapter(window);
    assert!(std::panic::catch_unwind(|| adapter(window)).is_err());
    dispatch(window, WM_SETFOCUS);
    assert_eq!(focuses(window), 1);
    drop(first);
}
#[test]
fn invalid_window_after_real_destruction_is_safe_to_drop() {
    let window = create(7);
    let owner = adapter(window);
    dispatch(window, WM_NCDESTROY);
    WINDOWS.with(|map| map.borrow_mut().get_mut(&7).unwrap().alive = false);
    drop(owner);
    assert_eq!(drops(window), 1);
}
#[test]
fn old_owner_cannot_remove_replacement_registration_after_detach_and_reuse() {
    let window = create(8);
    let old = adapter(window);
    dispatch(window, WM_NCDESTROY);
    WINDOWS.with(|map| map.borrow_mut().get_mut(&8).unwrap().alive = false);
    let reused = create(8);
    let current = adapter(reused);
    drop(old);
    dispatch(reused, WM_SETFOCUS);
    assert_eq!(focuses(reused), 1);
    drop(current);
    assert!(original_installed(reused));
}
struct DropDuringActivation {
    owner: Weak<RefCell<Option<subclass::SubclassingAdapter>>>,
    window: HWND,
}
impl ActivationHandler for DropDuringActivation {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        let owner = self.owner.upgrade().unwrap();
        drop(owner.borrow_mut().take());
        assert_eq!(
            drops(self.window),
            0,
            "current callback must retain state through user code"
        );
        Some(TreeUpdate)
    }
}
#[test]
fn reentrant_owner_drop_cannot_free_current_callback_state() {
    let window = create(9);
    let slot = Rc::new(RefCell::new(None));
    *slot.borrow_mut() = Some(subclass::SubclassingAdapter::new(
        window,
        DropDuringActivation {
            owner: Rc::downgrade(&slot),
            window,
        },
        NoAction,
    ));
    assert_eq!(dispatch(window, WM_GETOBJECT), LRESULT(19));
    assert!(slot.borrow().is_none());
    assert_eq!(drops(window), 1);
    assert!(original_installed(window));
}
#[test]
fn owner_thread_and_invalid_hwnd_are_rejected_before_native_install() {
    let window = create(10);
    WINDOWS.with(|map| map.borrow_mut().get_mut(&10).unwrap().thread = 2);
    assert!(std::panic::catch_unwind(|| adapter(window)).is_err());
    assert!(original_installed(window));
    assert_eq!(drops(window), 0);
    assert!(std::panic::catch_unwind(|| adapter(HWND(99usize as _))).is_err());
}

#[test]
fn recycled_hwnd_on_another_thread_is_never_rewritten_by_old_owner() {
    let window = create(11);
    let owner = adapter(window);
    // A later subclass leaves our forwarding registration resident.
    let previous = unsafe { SetWindowLongPtrW(window, GWLP_WNDPROC, later as *const () as isize) };
    LATER_PREVIOUS.with(|value| *value.borrow_mut() = Some(previous));
    dispatch(window, WM_NCDESTROY);
    // Simulate destruction then handle reuse by another owner thread. Its
    // installed generic callback can have the same address as our old callback.
    WINDOWS.with(|map| {
        let mut map = map.borrow_mut();
        let current = map.get_mut(&11).unwrap();
        current.thread = 2;
        current.procedure = previous;
    });
    drop(owner);
    assert_eq!(unsafe { GetWindowLongPtrW(window, GWLP_WNDPROC) }, previous);
    assert_eq!(drops(window), 1);
}
