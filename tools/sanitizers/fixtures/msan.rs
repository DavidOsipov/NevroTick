//! Isolated GNU/Linux detector control; not application code.
//! The seeded configuration deliberately violates a memory contract.
#![no_std]
#![no_main]

use core::ffi::{c_char, c_void};

#[link(name = "c")]
unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(pointer: *mut c_void);
    fn puts(message: *const c_char) -> i32;
    fn abort() -> !;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // No allocation or unwinding in the detector fixture's panic fallback.
    unsafe { abort() }
}

/// Consume heap data after initialization only in the healthy configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    let pointer = unsafe { malloc(core::mem::size_of::<i32>()) }.cast::<i32>();
    if pointer.is_null() { return 2; }
    #[cfg(not(seeded_defect))]
    unsafe { pointer.write_volatile(42); }
    // The seeded use is intentionally uninitialized; MSan must identify this site.
    let value = unsafe { pointer.read_volatile() };
    #[cfg(not(seeded_defect))]
    if value != 42 {
        unsafe { free(pointer.cast()); }
        return 3;
    }
    if value == 42 {
        unsafe { puts(c"SANITIZER_HEALTHY".as_ptr()); }
    } else {
        unsafe { puts(c"SANITIZER_DEFECT_REACHED".as_ptr()); }
    }
    unsafe { free(pointer.cast()); }
    0
}
