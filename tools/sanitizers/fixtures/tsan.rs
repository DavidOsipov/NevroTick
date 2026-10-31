//! Isolated GNU/Linux pthread detector control without an uninstrumented Rust std.
//! The seeded build deliberately races; the healthy build uses instrumented atomics.
#![no_std]
#![no_main]

use core::ffi::{c_char, c_void};
#[cfg(not(seeded_defect))]
use core::sync::atomic::{AtomicU64, Ordering};

#[link(name = "c")]
unsafe extern "C" {
    fn puts(message: *const c_char) -> i32;
    fn abort() -> !;
    fn pthread_create(thread: *mut usize, attributes: *const c_void,
                      entry: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      argument: *mut c_void) -> i32;
    fn pthread_join(thread: usize, result: *mut *mut c_void) -> i32;
}
#[cfg(seeded_defect)]
static mut SHARED: u64 = 0;
#[cfg(not(seeded_defect))]
static SHARED: AtomicU64 = AtomicU64::new(0);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! { unsafe { abort() } }

/// Both native threads reach this same operation under the selected configuration.
unsafe extern "C" fn worker(_: *mut c_void) -> *mut c_void {
    for _ in 0..1000 {
        #[cfg(seeded_defect)]
        unsafe {
            let pointer = &raw mut SHARED;
            pointer.write_volatile(pointer.read_volatile().wrapping_add(1));
        }
        #[cfg(not(seeded_defect))]
        SHARED.fetch_add(1, Ordering::SeqCst);
    }
    core::ptr::null_mut()
}

/// Join both successfully created threads before healthy-state observation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    let mut first = 0;
    let mut second = 0;
    if unsafe { pthread_create(&raw mut first, core::ptr::null(), worker, core::ptr::null_mut()) } != 0 { return 2; }
    if unsafe { pthread_create(&raw mut second, core::ptr::null(), worker, core::ptr::null_mut()) } != 0 {
        unsafe { pthread_join(first, core::ptr::null_mut()); }
        return 3;
    }
    if unsafe { pthread_join(first, core::ptr::null_mut()) } != 0 { return 4; }
    if unsafe { pthread_join(second, core::ptr::null_mut()) } != 0 { return 5; }
    #[cfg(not(seeded_defect))]
    if SHARED.load(Ordering::SeqCst) != 2000 { return 6; }
    #[cfg(not(seeded_defect))]
    unsafe { puts(c"SANITIZER_HEALTHY".as_ptr()); }
    #[cfg(seeded_defect)]
    unsafe { puts(c"SANITIZER_DEFECT_REACHED".as_ptr()); }
    0
}
