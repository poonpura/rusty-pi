#![no_std]  // No standard library
#![no_main] // No default entry point

mod start;

use core::panic::PanicInfo;
use libpi::gpio::*;
use libpi::time::*;

/// The entry point for the bare-metal kernel
#[no_mangle]
pub unsafe extern "C" fn notmain() -> ! {
    gpio_set_output(20);
    loop {
        gpio_set_on(20);  // Turn LED on
        delay_ms(1000);
        gpio_set_off(20); // Turn LED off
        delay_ms(1000);
    }
}

/// Panic handler (required because `no_std` removes the default panic handler)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}