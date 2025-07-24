
#![no_std]
#![no_main]
#![cfg(not(test))] //disable tests (because otherwise we rust analyzer gets naggy!)
use libc;

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe { libc::printf(c"Hello world!\n".as_ptr()) };
    0
}


#[allow(clippy::all)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}