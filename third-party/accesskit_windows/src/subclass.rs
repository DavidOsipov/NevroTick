// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit::{ActionHandler, ActivationHandler, TreeUpdate};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, btree_map::Entry},
    ffi::c_void,
    mem::transmute,
    rc::{Rc, Weak},
};
use windows::{
    Win32::{Foundation::*, System::Threading::GetCurrentThreadId, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::{Adapter, QueuedEvents};

fn win32_error() -> ! {
    panic!("{}", Error::from_thread())
}

// Work around a difference between the SetWindowLongPtrW API definition
// in windows-rs on 32-bit and 64-bit Windows.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
type LongPtr = isize;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
type LongPtr = i32;

// Window properties are writable by equal-integrity peer processes. They never
// select a Rust address or establish registration/lifetime in this adapter.
thread_local! {
    static SUBCLASSES: RefCell<BTreeMap<isize, Rc<Registration>>> = const {
        RefCell::new(BTreeMap::new())
    };
}

/// Process-owned chaining metadata; no strong adapter/handler ownership.
/// A later native subclass can require this forwarding record after owner Drop.
struct Registration {
    hwnd: HWND,
    previous: Cell<WNDPROC>,
    owner: Weak<SubclassImpl>,
    attached: Cell<bool>,
    active: Cell<bool>,
}

impl Registration {
    /// Remove only this generation, never a successor with a reused HWND.
    fn forget(self: &Rc<Self>) {
        SUBCLASSES.with(|subclasses| {
            let mut subclasses = subclasses.borrow_mut();
            if subclasses
                .get(&(self.hwnd.0 as isize))
                .is_some_and(|current| Rc::ptr_eq(current, self))
            {
                subclasses.remove(&(self.hwnd.0 as isize));
            }
        });
    }

    /// Owner-thread only. Detach our topmost procedure or retain safe forwarding
    /// beneath a later subclass. A message number is never proof of HWND death.
    fn detach_if_top(self: &Rc<Self>) {
        if !self.attached.get() {
            return;
        }
        let current_generation = SUBCLASSES.with(|subclasses| {
            subclasses
                .borrow()
                .get(&(self.hwnd.0 as isize))
                .is_some_and(|current| Rc::ptr_eq(current, self))
        });
        if !current_generation {
            return;
        }
        // SAFETY: Read-only observation of the registered HWND on its owner
        // thread. Rc prevents transferring this registration to another thread.
        if !unsafe { IsWindow(Some(self.hwnd)) }.as_bool() {
            self.attached.set(false);
            self.active.set(false);
            self.forget();
            return;
        }
        // A recycled handle can now belong to another thread, even while an
        // old forwarding registration is retained. It is no longer this
        // thread's window, so never restore a predecessor into that HWND.
        // SAFETY: Both native queries take no Rust-owned caller memory.
        if unsafe { GetWindowThreadProcessId(self.hwnd, None) } != unsafe { GetCurrentThreadId() } {
            self.attached.set(false);
            self.active.set(false);
            self.forget();
            return;
        }
        let our_proc = wnd_proc as *const c_void as LongPtr;
        // SAFETY: Current native WNDPROC is observed without calling it.
        if unsafe { GetWindowLongPtrW(self.hwnd, GWLP_WNDPROC) } != our_proc {
            return;
        }
        // SAFETY: The predecessor came from this same HWND's native installer;
        // this owner thread still observes our procedure at the chain head.
        let result = unsafe {
            SetWindowLongPtrW(
                self.hwnd,
                GWLP_WNDPROC,
                transmute::<WNDPROC, LongPtr>(self.previous.get()),
            )
        };
        if result == 0 {
            win32_error();
        }
        self.attached.set(false);
        self.active.set(false);
        self.forget();
    }
}

/// Retired chain metadata contains no live adapter; prune only observed invalid
/// HWNDs. Trusted adapter construction is the sole insertion path, never messages.
fn prune_retired() {
    SUBCLASSES.with(|subclasses| {
        subclasses.borrow_mut().retain(|_, registration| {
            registration.owner.strong_count() != 0
                // SAFETY: Each entry was installed on this TLS owner thread.
                || unsafe { IsWindow(Some(registration.hwnd)) }.as_bool()
        });
    });
}

struct SubclassState {
    adapter: Adapter,
    activation_handler: Box<dyn ActivationHandler>,
}

struct SubclassImpl {
    state: RefCell<SubclassState>,
    registration: Rc<Registration>,
}

extern "system" fn wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // Clone before any callback/native forwarding: no registry RefCell borrow
    // escapes lookup, and reentrant Drop cannot invalidate this owned frame.
    let registration =
        SUBCLASSES.with(|subclasses| subclasses.borrow().get(&(window.0 as isize)).cloned());
    let Some(registration) = registration else {
        // SAFETY: No process-owned registration exists; dereference no external
        // data and perform only the platform's default message processing.
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    };
    let previous = registration.previous.get();
    let owner = if registration.active.get() {
        registration.owner.upgrade()
    } else {
        None
    };
    if let Some(r#impl) = owner.as_ref() {
        match message {
            WM_GETOBJECT => {
                let mut state = r#impl.state.borrow_mut();
                let state_mut = &mut *state;
                if let Some(result) = state_mut.adapter.handle_wm_getobject(
                    wparam,
                    lparam,
                    &mut *state_mut.activation_handler,
                ) {
                    drop(state);
                    return result.into();
                }
            }
            WM_SETFOCUS | WM_EXITMENULOOP | WM_EXITSIZEMOVE => {
                r#impl.update_window_focus_state(true);
            }
            WM_KILLFOCUS | WM_ENTERMENULOOP | WM_ENTERSIZEMOVE => {
                r#impl.update_window_focus_state(false);
            }
            _ => (),
        }
    }
    if message == WM_NCDESTROY {
        // The message can be forged. Eagerly detach only when native chain-head
        // observation permits it; successful removal, not message receipt,
        // invalidates this registration. A later subclass is left intact.
        registration.detach_if_top();
    }
    // SAFETY: The owned registration retains the native predecessor and this
    // frame retains any upgraded adapter through all reentrant processing.
    unsafe { CallWindowProcW(previous, window, message, wparam, lparam) }
}

impl SubclassImpl {
    fn new(
        hwnd: HWND,
        activation_handler: impl 'static + ActivationHandler,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Rc<Self> {
        let adapter = Adapter::new(hwnd, false, action_handler);
        let state = RefCell::new(SubclassState {
            adapter,
            activation_handler: Box::new(activation_handler),
        });
        Rc::new_cyclic(|owner| Self {
            state,
            registration: Rc::new(Registration {
                hwnd,
                previous: Cell::new(None),
                owner: owner.clone(),
                attached: Cell::new(false),
                active: Cell::new(false),
            }),
        })
    }

    fn install(self: &Rc<Self>) {
        prune_retired();
        let registration = &self.registration;
        // SAFETY: Constructor verified the current thread owns this live HWND.
        let previous = unsafe { GetWindowLongPtrW(registration.hwnd, GWLP_WNDPROC) };
        if previous == 0 {
            win32_error();
        }
        // SAFETY: windows-rs models the native WNDPROC storage as LongPtr.
        registration
            .previous
            .set(unsafe { transmute::<LongPtr, WNDPROC>(previous) });
        SUBCLASSES.with(|subclasses| {
            match subclasses.borrow_mut().entry(registration.hwnd.0 as isize) {
                Entry::Vacant(entry) => {
                    entry.insert(Rc::clone(registration));
                }
                Entry::Occupied(_) => panic!(
                    "subclassing adapter already instantiated on window {:?}",
                    registration.hwnd.0
                ),
            }
        });
        registration.attached.set(true);
        registration.active.set(true);
        // SAFETY: Replace only this owner thread's HWND procedure. The process-
        // owned registration is already discoverable if USER32 reenters.
        let result = unsafe {
            SetWindowLongPtrW(
                registration.hwnd,
                GWLP_WNDPROC,
                wnd_proc as *const c_void as LongPtr,
            )
        };
        if result == 0 {
            registration.attached.set(false);
            registration.active.set(false);
            registration.forget();
            win32_error();
        }
        // SAFETY: The successful installer returns this HWND's actual predecessor.
        registration
            .previous
            .set(unsafe { transmute::<LongPtr, WNDPROC>(result) });
    }

    fn update_window_focus_state(&self, is_focused: bool) {
        let mut state = self.state.borrow_mut();
        if let Some(events) = state.adapter.update_window_focus_state(is_focused) {
            drop(state);
            events.raise();
        }
    }

    fn uninstall(&self) {
        self.registration.active.set(false);
        self.registration.detach_if_top();
        prune_retired();
    }
}

/// Uses [Win32 subclassing] to handle `WM_GETOBJECT` messages on a window
/// that provides no other way of adding custom message handlers.
///
/// [Win32 subclassing]: https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview
pub struct SubclassingAdapter(Rc<SubclassImpl>);

impl SubclassingAdapter {
    /// Creates a new Windows platform adapter using window subclassing.
    /// This must be done before the window is shown or focused
    /// for the first time.
    ///
    /// This must be called on the thread that owns the window. The activation
    /// handler will always be called on that thread. The action handler
    /// may or may not be called on that thread.
    ///
    /// # Panics
    ///
    /// Panics if the HWND is invalid, belongs to another thread, is already
    /// visible, or still has an owned registration (including a retired
    /// forwarding callback beneath another native subclass).
    pub fn new(
        hwnd: HWND,
        activation_handler: impl 'static + ActivationHandler,
        action_handler: impl 'static + ActionHandler + Send,
    ) -> Self {
        // SAFETY: Querying native window/thread identity has no caller-memory
        // precondition. Zero rejects invalid/foreign HWNDs before installation.
        let owner_thread = unsafe { GetWindowThreadProcessId(hwnd, None) };
        // SAFETY: This process/thread identity query takes no input.
        if owner_thread == 0 || owner_thread != unsafe { GetCurrentThreadId() } {
            panic!("The AccessKit Windows subclassing adapter requires the window owner thread.");
        }
        if unsafe { IsWindowVisible(hwnd) }.into() {
            panic!(
                "The AccessKit Windows subclassing adapter must be created before the window is shown (made visible) for the first time."
            );
        }

        let r#impl = SubclassImpl::new(hwnd, activation_handler, action_handler);
        r#impl.install();
        Self(r#impl)
    }

    /// While attached, if and only if the tree has been initialized, call the
    /// provided function and apply the resulting update. After a successful
    /// detach, return `None` without invoking the factory. Note: If the caller's implementation of
    /// [`ActivationHandler::request_initial_tree`] initially returned `None`,
    /// the [`TreeUpdate`] returned by the provided function must contain
    /// a full tree.
    ///
    /// If a [`QueuedEvents`] instance is returned, the caller must call
    /// [`QueuedEvents::raise`] on it.
    pub fn update_if_active(
        &mut self,
        update_factory: impl FnOnce() -> TreeUpdate,
    ) -> Option<QueuedEvents> {
        // SAFETY: We use `RefCell::borrow_mut` here, even though
        // `RefCell::get_mut` is allowed (because this method takes
        // a mutable self reference), just in case there's some way
        // this method can be called from within the subclassed window
        // procedure, e.g. via `ActivationHandler`.
        if !self.0.registration.active.get() {
            return None;
        }
        let mut state = self.0.state.borrow_mut();
        state.adapter.update_if_active(update_factory)
    }
}

impl Drop for SubclassingAdapter {
    fn drop(&mut self) {
        self.0.uninstall();
    }
}
