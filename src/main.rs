#![no_std]  // No standard library
#![no_main] // No default entry point

mod start;

use core::panic::PanicInfo;
use libpi::gpio::*;
// use libpi::time::*;
use libpi::uart::*;

/// The entry point for the bare-metal kernel
#[no_mangle]
pub unsafe extern "C" fn notmain() -> ! {
    uart_init();
    gpio_set_output(20);
    gpio_set_output(25);
    loop {
        uart_write("Light on from rusty-pi!\n");
        let c = uart_get8() as char;
        match c {
            'g' => match gpio_read(20) {
                1 => gpio_set_off(20),
                0 => gpio_set_on(20),
                _ => panic!("invalid bit!")
            }
            'r' => match gpio_read(25) {
                1 => gpio_set_off(25),
                0 => gpio_set_on(25),
                _ => panic!("invalid bit!")
            }
            o => {
                uart_write("Invalid key: ");
                uart_put8(o as u8);
                uart_put8('\n' as u8);
            }
        }
    }
}

/// Panic handler (required because `no_std` removes the default panic handler)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}