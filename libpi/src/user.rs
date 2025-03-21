#![allow(non_snake_case)]
/// Implementation for user threads for demo. 

use crate::gpio::*;
use crate::time::*;
use crate::uart::*;
use crate::threads::*;

const SYSTEM_CHECK_LOG: &str = r#"Commencing System Check
Memory Unit: Green
Initializing Tactics Log
Loading Geographic Data
Vitals: Green
Remaining MP: 100%
Black Box Temperature: Normal
Black Box Internal Pressure: Normal
Activating IFF
Activating FCS
Initializing Pod Connection
Launching DBU Setup
Activating Inertia Control System
Activating Environmental Sensors
Equipment Authentication: Complete
Equipment Status: Green
All Systems Green
Combat Preparations Complete_"#;

/// Toggles GPIO 25 (white) every second. Yields when done.
pub unsafe extern "C" fn threadA(i: u32) {
    let _ = i;
    loop {
        gpio_toggle(25);
        Timer::set(1000).wait_and_yield();
    }
}

/// Prints the `SYSTEM_CHECK_LOG` at a rate of 10B/s, but also yielding.
pub unsafe extern "C" fn threadB(i: u32) {
    let _ = i;
    Timer::set(1000).wait_and_yield();
    gpio_set_on(20);
    for c in SYSTEM_CHECK_LOG.chars() {
        uart_put8(c as u8);
        Timer::set(100).wait_and_yield();
    }
    gpio_set_off(20);
    exit_thread();
}

/// Prints a smiley face `i` times.
pub unsafe extern "C" fn threadC(i: u32) {
    for _ in 0..i {
        uart_print("\n(^_^)\n");
        Timer::set(1000).wait_and_yield(); 
    }
    exit_thread();
}

/// Spawns a threadC given char `c` that represents an ASCII digit.
#[allow(static_mut_refs)]
pub unsafe fn spawn(c: char) {
    let Some(i) = c.to_digit(10) else {
        panic!("c is not numeric!");
    };
    SCHEDULER.fork(threadC, i);
}