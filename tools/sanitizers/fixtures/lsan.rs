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

/// Return without retaining the allocation in the seeded build; otherwise free it.
#[inline(never)]
unsafe fn allocation() -> i32 {
    let pointer = unsafe { malloc(4096) }.cast::<u8>();
    if pointer.is_null() { return 2; }
    unsafe { pointer.write_volatile(42); }
    #[cfg(not(seeded_defect))]
    unsafe { free(pointer.cast()); }
    0
}

/// Ordinary process termination invokes the selected LeakSanitizer runtime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    let result = unsafe { allocation() };
    if result != 0 { return result; }
    #[cfg(not(seeded_defect))]
    unsafe { puts(c"SANITIZER_HEALTHY".as_ptr()); }
    #[cfg(seeded_defect)]
    unsafe { puts(c"SANITIZER_DEFECT_REACHED".as_ptr()); }
    0
}
