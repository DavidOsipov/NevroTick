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

/// The healthy build reads initialized storage; the seeded build reads past it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    let pointer = unsafe { malloc(8) }.cast::<u8>();
    if pointer.is_null() { return 2; }
    unsafe { pointer.write_volatile(17); }
    #[cfg(seeded_defect)]
    let index = 8;
    #[cfg(not(seeded_defect))]
    let index = 0;
    // The seeded access is intentional and must produce an ASan diagnosis.
    let value = unsafe { pointer.add(index).read_volatile() };
    unsafe { free(pointer.cast()); }
    if value != 17 { return 3; }
    unsafe { puts(c"SANITIZER_HEALTHY".as_ptr()); }
    0
}
