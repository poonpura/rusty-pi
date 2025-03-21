/// Simple timer related structs and operations. 

use core::arch::asm;
use crate::io::*;
use crate::threads::*;

const SYSTEM_TIMER: u32 = 0x20003004;

/// nop
#[inline(always)]
pub unsafe fn wait() {
    asm!("nop", options(nostack, preserves_flags));
}

/// Delays execution for `ms` milliseconds using the system timer.
pub unsafe fn delay_ms(ms: u32) {
    let start = get32(SYSTEM_TIMER);
    while get32(SYSTEM_TIMER).wrapping_sub(start) < (ms * 1000) {
        wait();
    }
}

/// A simple system timer object. 
pub struct Timer {
    stop_time: u32  
} 

impl Timer {
    /// Returns Timer struct that expires after `ms` milliseconds.
    pub unsafe fn timer(ms: u32) -> Self {
        Self {
            stop_time: get32(SYSTEM_TIMER).wrapping_add(ms * 1000)
        }
    }

    /// Returns `true` if timer has expired (`ms` milliseconds after `start()` called).
    pub unsafe fn done(&self) -> bool {
        self.stop_time.wrapping_sub(get32(SYSTEM_TIMER)) as i32 <= 0
    }

    /// Returns only when the timer has expired otherwise yields.
    pub unsafe fn wait_and_yield(&self) {
        while !self.done() {
            yield_thread();
        }
    }
}

