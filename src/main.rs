
#![no_std]
#![no_main]
#![cfg(not(test))] //disable tests (because otherwise we rust analyzer gets naggy!)

use theos::print;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print("Hello from print function!\n");
    print("This is a no_std print implementation.");

    loop {}
}


#[allow(clippy::all)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
