#![no_std]  // No standard library
#![no_main] // No default entry point

mod start;

use core::panic::PanicInfo;
use libpi::gpio::*;
use libpi::time::*;
use libpi::uart::*;
use libpi::threads::*;
use libpi::interrupts::*;
use libpi::user::*;

/// The entry point for the bare-metal kernel
#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn notmain() -> ! {
    uart_init();
    gpio_set_output(5);
    gpio_set_output(20);
    gpio_set_output(25);
    interrupt_init();
    gpio_rising_edge_init(21);
    SCHEDULER.fork(threadA, 5); // args don't matter 
    SCHEDULER.fork(threadB, 42); // args don't matter 
    SCHEDULER.cswitch();
    loop {
        wait();
    }
}

/// Panic handler (required because `no_std` removes the default panic handler)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}